import pytest
from e2e.core.base_helper import cli, BaseHelper


class ACLHelper:
    def __init__(self, cli, api_config):
        self._cli = cli
        self._api_config = api_config

    def set_acl(self, updates: dict):
        acl = self._cli.empty_acl()
        # update acl
        for key, value in updates.items():
            acl[0][key].append(value)
        self._cli.call(
            f"doc update {BaseHelper.TEST_CONFIG_NAME} {self._api_config['acl_setting']} /dev/stdin",
            inputjson=acl
        )

    def reset_and_set_acl(self, updates: dict):
        self._cli.revert_and_enable()
        self.set_acl(updates)
        self._cli.publish_and_apply()

    def reset_acl_to_default_values(self):
        default_values = {
            "allow": [],
            "allow_bot": [],
            "deny_bot": [],
            "passthrough": [],
            "deny": [],
            "force_deny": []
        }
        self.set_acl(default_values)


@pytest.fixture(scope="session")
def acl(cli, api_config):
    return ACLHelper(cli, api_config)
