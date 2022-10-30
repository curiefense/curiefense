import asyncio
import glob
import logging
import os
import re
from dataclasses import dataclass, make_dataclass
from datetime import datetime, timedelta, timezone
from time import time

import asyncify
import boto3
import more_itertools as mit
import requests
from google.api_core.exceptions import BadRequest
from google.cloud import bigquery, storage
from google.oauth2 import service_account

LOGLEVEL = os.getenv('INFUSER_BQ_LOGLEVEL', 'INFO').upper()
logging.basicConfig(level=LOGLEVEL)
logger = logging.getLogger('infuser-bq')
basic_metrics = {'service_name': 'infuser_bq'}

env_vars = [
    # timeout http requests using requests
    ['TIMEOUT_REQUESTS', 10],
    ['PROMEXPORTER_SERVICE_URI', 'http://promexporter:5000'],
    ['INFUSER_BQ_SA_PATH', None],
    ['INFUSER_BQ_PROJECT', None],
    ['INFUSER_BQ_DS', None],
    ['INFUSER_BQ_TABLE_ID', None],
    ['INFUSER_BQ_LOCATION', None],
    ['INFUSER_BQ_RAW_FILES_PATH_PATTERN', None],
    ['INFUSER_BQ_GCS_BUCKET_PATH', None],
    ['INFUSER_BQ_GCS_ACCESS_KEY_ID', None],
    ['INFUSER_BQ_GCS_SECRET_ACCESS_KEY', None],
    ['INFUSER_BQ_COMPOSE_FILES_COUNT', 32],
    ['INFUSER_BQ_RAW_FOLDER_PREFIX', 'raw'],
    ['INFUSER_BQ_COMPOSED_IN_PROCESS_FOLDER_PREFIX', 'in_process'],
    ['INFUSER_BQ_COMPOSED_DONE_FOLDER', 'done'],
    ['INFUSER_BQ_MAX_BAD_RECORDS', 100]
]


def load_config():
    config = make_dataclass('config', [])
    for name, default in env_vars:
        setattr(config, name.replace('INFUSER_BQ_', ''), os.getenv(name, default))

    gcs_bucket_name, gcs_bucket_folder = _parse_bucket_path(config.GCS_BUCKET_PATH)
    config.MINUTE_AGO            = datetime.now(timezone.utc) - timedelta(minutes=2)
    config.GCS_BUCKET_NAME       = gcs_bucket_name
    config.GCS_BUCKET_FOLDER     = gcs_bucket_folder
    config.GCP_CREDENTIALS       = service_account.Credentials.from_service_account_file(config.SA_PATH)
    config.DATE_PATH             = _build_date_path(config.MINUTE_AGO)

    config.COMPOSED_IN_PROCESS_FOLDER = f'{config.COMPOSED_IN_PROCESS_FOLDER_PREFIX}/{_build_date_suffix(config.MINUTE_AGO)}'
    config.GCS_RAW_FILES_FOLDER  = f'{config.GCS_BUCKET_FOLDER}{config.RAW_FOLDER_PREFIX}/{config.DATE_PATH}'
    config.IN_PROCESS_FOLDER     = f'{config.GCS_BUCKET_FOLDER}{config.COMPOSED_IN_PROCESS_FOLDER}'

    config.BQ_TABLE_PATH         = f'{config.PROJECT}.{config.DS}.{config.TABLE_ID}'
    config.BQ_LOAD_FROM_BUCKET   = f'{config.GCS_BUCKET_PATH}{config.COMPOSED_IN_PROCESS_FOLDER}/*'
    config.PROMEXPORTER_URI      = f'{config.PROMEXPORTER_SERVICE_URI}/metrics'
    return config


def init_clients(config):
    @dataclass
    class clients:
        GCS = storage.Client(credentials=config.GCP_CREDENTIALS)
        BQ  = bigquery.Client(credentials=config.GCP_CREDENTIALS)
    return clients


def send_metrics(config, metrics:list):
    """
    Send metrics dictionary to the metrics API
    """
    # return requests.post(url=config.PROMEXPORTER_URI,
    #                      json={**basic_metrics, 'data': metrics},
    #                      timeout=config.TIMEOUT_REQUESTS)


def _parse_bucket_path(bucket_path):
    match = re.match(r'(gs|s3)://([^/]+)/(.+)', bucket_path)
    bucket_name = match.group(2)
    path = match.group(3)
    return bucket_name, path


def _build_date_path(ts):
    return ts.strftime('%Y/%m/%d/%H/%M/')


def _build_date_suffix(ts):
    return ts.strftime("%Y%m%dT%H%M")


def _compose_file_path(config, suffix):
    return f'{config.IN_PROCESS_FOLDER}/{_build_date_suffix(config.MINUTE_AGO)}_{suffix}'


def get_source_files_groups(config, bucket):
    '''Break source small files into groups in case there are a lot'''
    source_files = bucket.list_blobs(prefix=config.GCS_RAW_FILES_FOLDER)
    # Clean from 'None's after grouper
    return list(map(lambda group: [f for f in group if f is not None],
                    # Break list of files into smaller lists in case there are too much files
                    mit.grouper(source_files, config.COMPOSE_FILES_COUNT)))


async def sync_local_to_gcs(config, bucket):
    @asyncify
    def _sync_file(file):
        path_upload_to = f'{config.GCS_RAW_FILES_FOLDER}{os.path.basename(file)}'
        logger.info(f'Upload file: {path_upload_to}')
        blob = bucket.blob(path_upload_to)
        blob.upload_from_filename(file)
        os.remove(file)
        logger.info(f'Local file {file} has been deleted')

    start_time = time()
    logger.info(f'Sync from local: {config.RAW_FILES_PATH_PATTERN}, into GCS: {config.GCS_RAW_FILES_FOLDER}')
    files_list = glob.glob(f'{config.RAW_FILES_PATH_PATTERN}')
    if not files_list:
        logger.warning(f'No files in the local bucket: {config.RAW_FILES_PATH_PATTERN} to sync')
        exit(0)
    await asyncio.gather(*map(_sync_file, files_list))
    send_metrics(config, [{'sync_s3_to_gcp_time': time() - start_time},
                          {'sync_s3_to_gcp_files': len(files_list)}])
    return files_list


async def compose_files(config, bucket):
    @asyncify
    def _compose(suffix, files):
        compose_file_path = _compose_file_path(config, suffix=suffix)
        logger.info(f'Compose into file: {compose_file_path}')
        start_time = time()
        destination = bucket.blob(compose_file_path)
        destination.content_type = "json"
        destination.compose(files)
        # Clean up source files
        logger.info(f'Remove source files: {[f.name for f in files]}')
        bucket.delete_blobs(files)
        send_metrics(config, [{'compose_files_time': time() - start_time},
                              {'compose_files_files': len(files)}])

    files_groups = get_source_files_groups(config, bucket)
    await asyncio.gather(*map(lambda i_fg: _compose(*i_fg), enumerate(files_groups)))


def load_from_bucket_2_bq(config, clients):
    start_time = time()
    logger.info(f'Push files from {config.BQ_LOAD_FROM_BUCKET} into BQ table: {config.BQ_TABLE_PATH}')
    table = clients.BQ.get_table(config.BQ_TABLE_PATH)
    job_config = bigquery.LoadJobConfig(source_format=bigquery.SourceFormat.NEWLINE_DELIMITED_JSON,
                                        schema=table.schema,
                                        ignore_unknown_values=True,
                                        max_bad_records=config.MAX_BAD_RECORDS)
    load_job = clients.BQ.load_table_from_uri(config.BQ_LOAD_FROM_BUCKET,
                                              config.BQ_TABLE_PATH,
                                              location=config.LOCATION,
                                              job_config=job_config)
    try:
        load_job.result()
        if load_job.errors:
            logger.warning(f'Ignored errors during BQ push: {load_job.errors}')
        send_metrics(config, [{'bq_load_time': time() - start_time},
                              {'bq_load_files': load_job.input_files},
                              {'bq_load_rows': load_job.output_rows},
                              {'bq_load_errors': len(load_job.errors or [])}])
    except BadRequest as e:
        logger.exception(load_job.errors)
        raise e


async def move_to_archive(config, bucket):
    '''Move uploaded to BQ composed files to archive'''
    @asyncify
    def _move(file):
        logger.info(f'Move uploaded composed file {file.name} to archive')
        new_name = file.name.replace(config.COMPOSED_IN_PROCESS_FOLDER,
                                     config.COMPOSED_DONE_FOLDER)
        bucket.rename_blob(file, new_name=new_name)

    start_time = time()
    files_in_progress = list(bucket.list_blobs(prefix=config.IN_PROCESS_FOLDER))
    if files_in_progress:
        await asyncio.gather(*map(_move, files_in_progress))

    send_metrics(config, [{'move_to_archive_time': time() - start_time},
                          {'move_to_archive_files': len(files_in_progress)}])


async def main():
    config = load_config()
    clients = init_clients(config)
    logger.info(f'Start infuser for logs on date: {config.MINUTE_AGO.strftime("%Y-%m-%d %H:%M")}')

    bucket = clients.GCS.get_bucket(config.GCS_BUCKET_NAME)

    await sync_local_to_gcs(config, bucket)
    await compose_files(config, bucket)
    load_from_bucket_2_bq(config, clients)
    await move_to_archive(config, bucket)
    logger.info('Done')


if __name__ == '__main__':
    asyncio.run(main())
