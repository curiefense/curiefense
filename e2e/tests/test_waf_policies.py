import pytest
from e2e.core.base_helper import cli, target, section, default_config, BaseHelper
from e2e.helpers.acl_helper import acl
from e2e.helpers.waf_policies_helper import wafparam_config, ignore_alphanum, name_regex, restrict


@pytest.mark.usefixtures('api_setup', 'default_config')
@pytest.mark.all_modules
@pytest.mark.waf_policies_tests
class TestWAFLengthCount:
    def test_length_overlong(self, target, section):
        # default limit: len 1024
        response = target.query(
            f"/overlong-{section}",
            **{section: {f"Long-{section}": f"Overlong_{section}" * 100}},
        )
        assert response.status_code == 403, f"Reachable despite overlong {section}"

    def test_length_short(self, target, section):
        assert target.is_reachable(
            f"/short-{section}", headers={f"Short-{section}": f"Short_{section}"}
        ), f"Not reachable despite short {section}"

    def test_count_few(self, target, section):
        # default limit: 512 for args, 42 for other sections
        values = {}
        for i in range(10):
            values[f"{section}-{i}"] = "not_alphanum"
        assert target.is_reachable(
            f"/few-{section}", **{section: values}
        ), f"Not reachable despite few {section}"

    def test_count_toomany(self, target, section):
        values = {}
        for i in range(513):
            values[f"{section}-{i}"] = "not_alphanum"
        assert not target.is_reachable(
            f"/too-many-{section}", **{section: values}
        ), f"Reachable despite too many {section}"


@pytest.mark.usefixtures('api_setup', 'wafparam_config')
@pytest.mark.waf_policies_tests
@pytest.mark.all_modules
class TestWAFParamsConstraints:
    def test_allowlisted_value(
        self, section, name_regex, restrict, target
    ):
        paramname = name_regex + "-" + restrict
        assert target.is_reachable(
            f"/allowlisted-value-{paramname}", **{section: {paramname: "value"}}
        ), f"Not reachable despite allowlisted {section} value"

    def test_non_allowlisted_value_restrict(
        self, section, name_regex, target, ignore_alphanum
    ):
        paramname = name_regex + "-restrict"
        if ignore_alphanum:
            assert target.is_reachable(
                f"/blocklisted-value-{paramname}-restrict-ignore_alphanum",
                **{section: {paramname: "invalid"}},
            ), f"Not reachable despite alphanum blocklisted {section} value (restrict is enabled)"
        else:
            assert not target.is_reachable(
                f"/blocklisted-value-{paramname}-restrict",
                **{section: {paramname: "invalid"}},
            ), f"Reachable despite blocklisted {section} value (restrict is enabled)"

    def test_non_allowlisted_value_norestrict_nowafmatch(
        self, section, name_regex, target
    ):
        paramname = name_regex + "-norestrict"
        assert target.is_reachable(
            f"/blocklisted-value-{paramname}", **{section: {paramname: "invalid"}}
        ), f"Not reachable despite 'restricted' not checked (non-matching {section} value)"

    def test_non_allowlisted_value_norestrict_wafmatch(
        self, section, name_regex, target
    ):
        paramname = name_regex + "-norestrict"
        assert not target.is_reachable(
            f"/blocklisted-value-{paramname}-wafmatch",
            **{section: {paramname: "../../../../../"}},
        ), f"Reachable despite matching wafsig 100116 (non-matching {section} value)"

    def test_non_allowlisted_value_norestrict_wafmatch_excludesig(
        self, section, name_regex, target
    ):
        paramname = name_regex + "-norestrict"
        assert target.is_reachable(
            f"/blocklisted-value-{paramname}-wafmatch-excludedsig",
            **{section: {paramname: "htaccess"}},
        ), f"Not reachable despite excludesig for rule 100140 ({section} value)"

