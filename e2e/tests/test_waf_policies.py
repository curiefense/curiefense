import pytest
from e2e.core.base_helper import cli, target, section, default_config, BaseHelper
from e2e.helpers.acl_helper import acl
from e2e.helpers.waf_policies_helper import name_regex, restrict, waf_test_config, ignore_alphanum


@pytest.mark.usefixtures('api_setup', 'waf_test_config')
@pytest.mark.all_modules
@pytest.mark.waf_policies_tests
class TestWAFPolicies:

    def test_section_overlong_length_with_default_value(self, target, section):
        response = target.query(
            f"/long-{section}",
            **{section: {f"Long-{section}": f"Overlong_{section}" * 100}}
        )
        assert response.status_code == 403, f"Response not 403 despite overlong {section}"

    def test_section_short_length_with_default_value(self, target, section):
        response = target.query(
            f"/short-{section}",
            **{section: {f"Short-{section}": f"Short_{section}" * 10}}
        )
        assert response.status_code == 200, f"Response not 200 despite correct length {section}"

    def test_section_overlong_length_with_short_value(self, target, section):
        response = target.query(
            f"/test_section_overlong_length_with_short_value/long-{section}",
            **{section: {f"Long-{section}": f"Overlong_{section}" * 10}}
        )
        assert response.status_code == 403, f"Response not 403 despite overlong {section}"

    def test_section_short_length_with_short_value(self, target, section):
        response = target.query(
            f"/test_section_short_length_with_short_value/short-{section}",
            **{section: {f"Short-{section}": f"Short{section}" * 2}}
        )
        assert response.status_code == 200, f"Response not 200 despite correct length {section}"

    @pytest.mark.parametrize("length", [10, 512, 513])
    def test_args_count_default(self, target, length):
        values = {}
        for i in range(length):
            values[f"args-{i}"] = "alphanum"
        response = target.query(f"/few", **{"params": values})
        if length <= 512:
            assert response.status_code == 200, f"Got {response.status_code} instead of 200"
        else:
            assert response.status_code == 403, f"Got {response.status_code} instead of 403"

    @pytest.mark.parametrize("length", [8, 9, 10])
    def test_low_args_counter(self, target, length):
        values = {}
        for i in range(length):
            values[f"args-{i}"] = "alphanum"
        response = target.query(f"/test_low_args_counter", **{"params": values})
        if length <= 9:
            assert response.status_code == 200, f"Got {response.status_code} instead of 200"
        else:
            assert response.status_code == 403, f"Got {response.status_code} instead of 403"

    @pytest.mark.parametrize("length, sect", [(10, "headers"), (10, "cookies"),
                                              (35, "headers"), (42, "cookies"),
                                              (43, "headers"), (43, "cookies")])
    def test_headers_cookies_count_default(self, target, length, sect):
        values = {}
        for i in range(length):
            values[f"{sect}-{i}"] = "alphanum"
        response = target.query(f"/{sect}", **{sect: values})
        if length <= 42:
            assert response.status_code == 200, f"Got {response.status_code} instead of 200"
        else:
            assert response.status_code == 403, f"Got {response.status_code} instead of 403"

    @pytest.mark.parametrize("length, sect", [(12, "headers"), (18, "cookies"),
                                              (13, "headers"), (19, "cookies"),
                                              (21, "headers"), (21, "cookies")])
    def test_low_headers_cookies_counter(self, target, length, sect):
        values = {}
        for i in range(length):
            values[f"{sect}-{i}"] = f"alphanum"
        response = target.query(f"/test_low_headers_cookies_counter", **{sect: values})
        if length <= 20:
            assert response.status_code == 200, f"Got {response.status_code} instead of 200"
        else:
            assert response.status_code == 403, f"Got {response.status_code} instead of 403"

    @pytest.mark.parametrize("uri, sect, block_name, pass_name", [("abcd", "cookies", "sasa", "block_cookie"),
                                                                  ("bcde", "headers", "erer", "block_header"),
                                                                  ("cdef", "params", "bnbnb", "block_arg")])
    def test_block_cookie_header_arg_by_name(self, target, uri, sect, block_name, pass_name):
        response = target.query(
            f"/{uri}",
            **{sect: {"foobar": block_name}}
        )
        assert response.status_code == 403, f"Response not 403 despite {sect} does not match pattern"
        response = target.query(
            f"/{uri}",
            **{sect: {"foobar": pass_name}}
        )
        assert response.status_code == 200, f"Response not 200 despite {sect} matches pattern"

    @pytest.mark.parametrize("uri, sect, block_name", [("ig_alpha_cook", "cookies", "sasa"),
                                                       ("ig_alph_head", "headers", "erer"),
                                                       ("ig_alph_arg", "params", "bnbnb")])
    def test_ignore_alphanumeric(self, target, uri, sect, block_name):
        response = target.query(
            f"/{uri}",
            **{sect: {"foobar": block_name}}
        )
        assert response.status_code == 200, "Response not 200 despite ignore alphanumeric is True"


    def test_allowlisted_value(
            self, section, name_regex, restrict, target
    ):
        paramname = name_regex + "-" + restrict
        assert target.is_reachable(
            f"/allowlisted-value-{paramname}", **{section: {paramname: "value"}}
        ), f"Not reachable despite allowlisted {section} value"

# @pytest.mark.usefixtures('api_setup', 'waf_test_config')
# @pytest.mark.waf_policies_tests
# @pytest.mark.all_modules
# class TestWAFParamsConstraints:

# def test_allowlisted_value(
#     self, section, name_regex, restrict, target
# ):
#     paramname = name_regex + "-" + restrict
#     assert target.is_reachable(
#         f"/allowlisted-value-{paramname}", **{section: {paramname: "value"}}
#     ), f"Not reachable despite allowlisted {section} value"
#
# def test_non_allowlisted_value_restrict(
#     self, section, name_regex, target, ignore_alphanum
# ):
#     paramname = name_regex + "-restrict"
#     if ignore_alphanum:
#         assert target.is_reachable(
#             f"/blocklisted-value-{paramname}-restrict-ignore_alphanum",
#             **{section: {paramname: "invalid"}},
#         ), f"Not reachable despite alphanum blocklisted {section} value (restrict is enabled)"
#     else:
#         assert not target.is_reachable(
#             f"/blocklisted-value-{paramname}-restrict",
#             **{section: {paramname: "invalid"}},
#         ), f"Reachable despite blocklisted {section} value (restrict is enabled)"
# #
# def test_non_allowlisted_value_norestrict_nowafmatch(
#     self, section, name_regex, target
# ):
#     paramname = name_regex + "-norestrict"
#     assert target.is_reachable(
#         f"/blocklisted-value-{paramname}", **{section: {paramname: "invalid"}}
#     ), f"Not reachable despite 'restricted' not checked (non-matching {section} value)"
#
# def test_non_allowlisted_value_norestrict_wafmatch(
#     self, section, name_regex, target
# ):
#     paramname = name_regex + "-norestrict"
#     assert not target.is_reachable(
#         f"/blocklisted-value-{paramname}-wafmatch",
#         **{section: {paramname: "../../../../../"}},
#     ), f"Reachable despite matching wafsig 100116 (non-matching {section} value)"
#
# def test_non_allowlisted_value_norestrict_wafmatch_excludesig(
#     self, section, name_regex, target
# ):
#     paramname = name_regex + "-norestrict"
#     assert target.is_reachable(
#         f"/blocklisted-value-{paramname}-wafmatch-excludedsig",
#         **{section: {paramname: "htaccess"}},
#     ), f"Not reachable despite excludesig for rule 100140 ({section} value)"
#
