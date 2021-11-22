import json
from locust import HttpUser, task, between, events


@events.init_command_line_parser.add_listener
def _(parser):
    parser.add_argument(
        "--cf-reqsize",
        type=str,
        env_var="LOCUST_CF_REQSIZE",
        default="",
        help="Request size (kB)",
    )
    parser.add_argument(
        "--cf-testid",
        type=str,
        env_var="LOCUST_CF_TESTID",
        default="default",
        help="Test identifier",
    )


class Curiefense(HttpUser):
    wait_time = between(0.1, 0.2)

    def on_start(self):
        self.client.verify = False

    def gen_payload(self, size_kb: int):
        if size_kb == 0:
            return ""
        else:
            return json.dumps(
                [
                    # 100 bytes after json encoding
                    {"contents": "testcontent " * 7},
                ]
                * (size_kb * 10 - 1)
                + [
                    # 100 bytes after json encoding, including surrounding list
                    {"content": "aa5%3Cimg%20src=1%20onerror=console.log(4)%3E"},
                    {"padding": "A" * 21},
                ]
            )

    @task(1)
    def curiefense_perf3(self):
        reqsize = int(self.environment.parsed_options.cf_reqsize)
        headers = {}
        if reqsize >= 1:
            headers = {
                "User-Agent": "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/85.0.4183.102 Safari/537.36",
                "Accept": "text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.9",
                "Cookie": "inst=APP_4L;BrowserId=AAAAAAAAAAAAAAAAAAAAAA;BrowserId_sec=BBBBBBBBBBBBBBBBBBBBBB; YYY-stream=CCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCC; force-proxy-stream=DDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDD; force-stream=EEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEE; sid=FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF; sid_Client=GGGGGGGGGGGGGGGGGGGGGG;clientSrc=1.2.3.4",
                "Accept-Encoding": "gzip, deflate",
                "Accept-Language": "en-US,en;q=0.9",
                "Cache-Control": "no-cache",
                "Pragma": "no-cache",
                "Sec-Fetch-Dest": "document",
                "Sec-Fetch-Mode": "navigate",
                "Sec-Fetch-Site": "none",
                "Sec-Fetch-User": "?1",
                "Upgrade-Insecure-Requests": "1",
                "Content-Type": "application/json",
            }
            # header size:
            # len("\n".join([f"{k}: {v}" for k, v in headers.items()])) = 1000
        payload = self.gen_payload(size_kb=reqsize - 1)
        with self.client.post(
            f"/invalid/{self.environment.parsed_options.cf_testid}",
            data=payload,
            headers=headers,
            allow_redirects=False,
            catch_response=True,
        ) as response:
            if response.status_code not in [403, 404, 483]:
                response.success()
            else:
                response.failure(
                    "Did not get expected code:" + str(response.status_code)
                )
