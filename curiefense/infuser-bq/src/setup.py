from setuptools import find_packages, setup

setup(
    name="bq",
    version="0.1.0",
    description="Curiefense pipe from s3 bucket to mongodb",
    author="Reblaze",
    packages=find_packages(),
    scripts=["bin/infuserbqctl"],
    install_requires=[
        "google-cloud-bigquery==3.3.3",
        "google-cloud-bigquery-storage==2.16.1",
        "google-cloud-storage==2.5.0",
        "boto3==1.24.89",
        "more_itertools==8.14.0",
        "asyncify==0.9.1"
    ],
)