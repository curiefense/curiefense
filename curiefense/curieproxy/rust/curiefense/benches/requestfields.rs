use criterion::*;
use curiefense::config::contentfilter::Transformation;
use curiefense::config::utils::DataSource;
use curiefense::requestfields::RequestField;

static ENTITIES: [&str; 7] = ["&quot;", "&amp;", "&lt;", "&gt;", "&nbsp;", "&apos;", "&#128512;"];
static UNICODE: [&str; 5] = [
    "<DIV STYLE=\"background-image:\\0075\\0072\\006C\\0028'\\006a\\0061\\0076\\0061\\0073\\0063\\0072\\0069\\0070",
    "\\0020\\u0020\\U00000020",
    "\\uD83D\\uDC36",
    "\\D83D\\uDC36",
    "\\uD83D\\uDC69\\u200D\\u2764\\uFE0F\\u200D\\uD83D\\uDC69",
];

fn rf_test(decoding: &[Transformation], str: &str) {
    let rf = RequestField::singleton(decoding, "key".to_string(), DataSource::Root, str.to_string());
    assert!(!rf.fields.is_empty())
}

fn ascii_nofilter(c: &mut Criterion) {
    let mut group = c.benchmark_group("ASCII nofilter");
    for sz in [100, 10000].iter() {
        let mut str = String::new();
        for x in 0..*sz {
            str += format!("{}", x).as_str();
            if str.len() > *sz {
                break;
            }
        }
        group.bench_with_input(BenchmarkId::from_parameter(str.len()), sz, |b, &_| {
            b.iter(|| rf_test(&[], black_box(&str)))
        });
    }
}

fn ascii_allfilters(c: &mut Criterion) {
    use Transformation::*;
    let mut group = c.benchmark_group("ASCII all filters");
    for sz in [100, 10000].iter() {
        let mut str = String::new();
        for x in 0..*sz {
            str += format!("{}", x).as_str();
            if str.len() > *sz {
                break;
            }
        }
        group.bench_with_input(BenchmarkId::from_parameter(str.len()), sz, |b, &_| {
            b.iter(|| {
                rf_test(
                    &[Base64Decode, UrlDecode, HtmlEntitiesDecode, UnicodeDecode],
                    black_box(&str),
                )
            })
        });
    }
}

fn html_nofilter(c: &mut Criterion) {
    let mut group = c.benchmark_group("HTML nofilter");
    for sz in [100, 10000].iter() {
        let mut str = String::new();
        for x in 0..*sz {
            str += ENTITIES[x % ENTITIES.len()];
            if str.len() > *sz {
                break;
            }
        }
        group.bench_with_input(BenchmarkId::from_parameter(str.len()), sz, |b, &_| {
            b.iter(|| rf_test(&[], black_box(&str)))
        });
    }
}

fn html_allfilters(c: &mut Criterion) {
    use Transformation::*;
    let mut group = c.benchmark_group("HTML all filters");
    for sz in [100, 10000].iter() {
        let mut str = String::new();
        for x in 0..*sz {
            str += ENTITIES[x % ENTITIES.len()];
            if str.len() > *sz {
                break;
            }
        }
        group.bench_with_input(BenchmarkId::from_parameter(str.len()), sz, |b, &_| {
            b.iter(|| {
                rf_test(
                    &[Base64Decode, UrlDecode, HtmlEntitiesDecode, UnicodeDecode],
                    black_box(&str),
                )
            })
        });
    }
}

fn unicode_nofilter(c: &mut Criterion) {
    let mut group = c.benchmark_group("Unicode nofilter");
    for sz in [100, 10000].iter() {
        let mut str = String::new();
        for x in 0..*sz {
            str += UNICODE[x % UNICODE.len()];
            if str.len() > *sz {
                break;
            }
        }
        group.bench_with_input(BenchmarkId::from_parameter(str.len()), sz, |b, &_| {
            b.iter(|| rf_test(&[], black_box(&str)))
        });
    }
}

fn unicode_allfilters(c: &mut Criterion) {
    use Transformation::*;
    let mut group = c.benchmark_group("Unicode all filters");
    for sz in [100, 10000].iter() {
        let mut str = String::new();
        for x in 0..*sz {
            str += UNICODE[x % UNICODE.len()];
            if str.len() > *sz {
                break;
            }
        }
        group.bench_with_input(BenchmarkId::from_parameter(str.len()), sz, |b, &_| {
            b.iter(|| {
                rf_test(
                    &[Base64Decode, UrlDecode, HtmlEntitiesDecode, UnicodeDecode],
                    black_box(&str),
                )
            })
        });
    }
}

criterion_group!(ascii, ascii_nofilter, ascii_allfilters);
criterion_group!(html, html_nofilter, html_allfilters);
criterion_group!(unicode, unicode_nofilter, unicode_allfilters);
criterion_main!(ascii, html, unicode);
