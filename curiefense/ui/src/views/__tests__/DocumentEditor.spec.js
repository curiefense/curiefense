import DocumentEditor from '@/views/DocumentEditor'
import {afterEach, beforeEach, describe, expect, jest, test} from '@jest/globals'
import {shallowMount} from '@vue/test-utils'
import GitHistory from '@/components/GitHistory'
import Vue from 'vue'
import DatasetsUtils from '@/assets/DatasetsUtils'
import axios from 'axios'
import Utils from '@/assets/Utils'

jest.mock('axios')

describe('DocumentEditor.vue', () => {
    let wrapper
    let mockRoute
    let mockRouter
    let gitData
    let aclDocs
    let aclDocsLogs
    let aclGitOldVersion
    let profilingListDocs
    let profilingListDocsLogs
    let urlMapsDocs
    let urlMapsDocsLogs
    let flowControlDocs
    beforeEach((done) => {
        gitData = [
            {
                'id': 'master',
                'description': 'Update entry [__default__] of document [aclpolicies]',
                'date': '2020-11-10T15:49:17+02:00',
                'logs': [
                    {
                        'version': '7dd9580c00bef1049ee9a531afb13db9ef3ee956',
                        'date': '2020-11-10T15:49:17+02:00',
                        'parents': [
                            'fc47a6cd9d7f254dd97875a04b87165cc484e075'
                        ],
                        'message': 'Update entry [__default__] of document [aclpolicies]',
                        'email': 'curiefense@reblaze.com',
                        'author': 'Curiefense API'
                    },
                    {
                        'version': 'fc47a6cd9d7f254dd97875a04b87165cc484e075',
                        'date': '2020-11-10T15:48:35+02:00',
                        'parents': [
                            '5aba4a5b9d6faea1896ee8965c7aa651f76af63c'
                        ],
                        'message': 'Update entry [__default__] of document [aclpolicies]',
                        'email': 'curiefense@reblaze.com',
                        'author': 'Curiefense API'
                    },
                    {
                        'version': '5aba4a5b9d6faea1896ee8965c7aa651f76af63c',
                        'date': '2020-11-10T15:48:31+02:00',
                        'parents': [
                            '277c5d7bd0e2eb4b9d2944f7eefdfadf37ba8581'
                        ],
                        'message': 'Update entry [__default__] of document [aclpolicies]',
                        'email': 'curiefense@reblaze.com',
                        'author': 'Curiefense API'
                    },
                    {
                        'version': '277c5d7bd0e2eb4b9d2944f7eefdfadf37ba8581',
                        'date': '2020-11-10T15:48:22+02:00',
                        'parents': [
                            '878b47deeddac94625fe7c759786f2df885ec541'
                        ],
                        'message': 'Update entry [__default__] of document [aclpolicies]',
                        'email': 'curiefense@reblaze.com',
                        'author': 'Curiefense API'
                    },
                    {
                        'version': '878b47deeddac94625fe7c759786f2df885ec541',
                        'date': '2020-11-10T15:48:05+02:00',
                        'parents': [
                            '93c180513fe7edeaf1c0ca69a67aa2a11374da4f'
                        ],
                        'message': 'Update entry [__default__] of document [aclpolicies]',
                        'email': 'curiefense@reblaze.com',
                        'author': 'Curiefense API'
                    },
                    {
                        'version': '93c180513fe7edeaf1c0ca69a67aa2a11374da4f',
                        'date': '2020-11-10T15:47:59+02:00',
                        'parents': [
                            '1662043d2a18d6ad2c9c94d6f826593ff5506354'
                        ],
                        'message': 'Update entry [__default__] of document [aclpolicies]',
                        'email': 'curiefense@reblaze.com',
                        'author': 'Curiefense API'
                    },
                    {
                        'version': '1662043d2a18d6ad2c9c94d6f826593ff5506354',
                        'date': '2020-11-08T21:31:41+01:00',
                        'parents': [
                            '16379cdf39501574b4a2f5a227b82a4454884b84'
                        ],
                        'message': 'Create config [master]\n',
                        'email': 'curiefense@reblaze.com',
                        'author': 'Curiefense API'
                    },
                    {
                        'version': '16379cdf39501574b4a2f5a227b82a4454884b84',
                        'date': '2020-08-27T16:19:06+00:00',
                        'parents': [
                            'a34f979217215060861b58b3f270e82580c20efb'
                        ],
                        'message': 'Initial empty config',
                        'email': 'curiefense@reblaze.com',
                        'author': 'Curiefense API'
                    },
                    {
                        'version': 'a34f979217215060861b58b3f270e82580c20efb',
                        'date': '2020-08-27T16:19:06+00:00',
                        'parents': [],
                        'message': 'Initial empty content',
                        'email': 'curiefense@reblaze.com',
                        'author': 'Curiefense API'
                    }
                ],
                'version': '7dd9580c00bef1049ee9a531afb13db9ef3ee956'
            },
            {
                'id': 'zzz_branch',
                'description': 'Initial empty content',
                'date': '2020-08-27T16:19:06+00:00',
                'logs': [
                    {
                        'version': 'a34f979217215060861b58b3f270e82580c20efb',
                        'date': '2020-08-27T16:19:06+00:00',
                        'parents': [],
                        'message': 'Initial empty content',
                        'email': 'curiefense@reblaze.com',
                        'author': 'Curiefense API'
                    }
                ],
                'version': 'a34f979217215060861b58b3f270e82580c20efb'
            }
        ]
        aclDocs = [
            {
                'id': '__default__',
                'name': 'default-acl',
                'allow': [],
                'allow_bot': [
                    'google'
                ],
                'deny_bot': [],
                'bypass': [
                    'internal'
                ],
                'deny': [
                    'tor'
                ],
                'force_deny': [
                    'china'
                ]
            },
            {
                'id': '5828321c37e0',
                'name': 'an ACL',
                'allow': [],
                'allow_bot': [
                    'google',
                    'yahoo',
                ],
                'deny_bot': [],
                'bypass': [
                    'devops'
                ],
                'deny': [
                    'tor'
                ],
                'force_deny': [
                    'iran'
                ]
            }
        ]
        aclDocsLogs = [
            [
                {
                    'version': '7f8a987c8e5e9db7c734ac8841c543d5bc5d9657',
                    'date': '2020-11-12T17:23:11+02:00',
                    'parents': [
                        '82d8f29096af1db07dbf7e1cff581fdf6e1a7440'
                    ],
                    'message': 'Revert to version [7a24bd37e93e812fa5173c4b2fb0068ad8e4ffdd]',
                    'email': 'curiefense@reblaze.com',
                    'author': 'Curiefense API'
                },
                {
                    'version': '82d8f29096af1db07dbf7e1cff581fdf6e1a7440',
                    'date': '2020-11-12T17:22:56+02:00',
                    'parents': [
                        '7a24bd37e93e812fa5173c4b2fb0068ad8e4ffdd'
                    ],
                    'message': 'Update entry [__default__] of document [aclpolicies]',
                    'email': 'curiefense@reblaze.com',
                    'author': 'Curiefense API'
                },
                {
                    'version': '7a24bd37e93e812fa5173c4b2fb0068ad8e4ffdd',
                    'date': '2020-11-12T17:22:53+02:00',
                    'parents': [
                        '886baf66ddd032744ac34b848c8412386a160fb3'
                    ],
                    'message': 'Update entry [__default__] of document [aclpolicies]',
                    'email': 'curiefense@reblaze.com',
                    'author': 'Curiefense API'
                },
                {
                    'version': '886baf66ddd032744ac34b848c8412386a160fb3',
                    'date': '2020-11-12T17:22:51+02:00',
                    'parents': [
                        'af98cb28fc4db3a76c3a51d697b6037e8695dd7b'
                    ],
                    'message': 'Update entry [__default__] of document [aclpolicies]',
                    'email': 'curiefense@reblaze.com',
                    'author': 'Curiefense API'
                },
                {
                    'version': 'af98cb28fc4db3a76c3a51d697b6037e8695dd7b',
                    'date': '2020-11-12T17:22:50+02:00',
                    'parents': [
                        'cda70058b632405600db1fbc5cf8dfd90514ec30'
                    ],
                    'message': 'Update entry [__default__] of document [aclpolicies]',
                    'email': 'curiefense@reblaze.com',
                    'author': 'Curiefense API'
                },
                {
                    'version': 'e57349cabbb31dd4f07945a42c2ab91671a5a7b1',
                    'date': '2020-11-10T16:00:18+02:00',
                    'parents': [
                        '78d2193cdaa2818734894ab5bbb85cf932d4f217'
                    ],
                    'message': 'Revert document [aclpolicies] to version [6c439c1626b15011c8eac9117026e0bb4c9f3a1e]',
                    'email': 'curiefense@reblaze.com',
                    'author': 'Curiefense API'
                },
                {
                    'version': '78d2193cdaa2818734894ab5bbb85cf932d4f217',
                    'date': '2020-11-10T16:00:18+02:00',
                    'parents': [
                        '6c439c1626b15011c8eac9117026e0bb4c9f3a1e'
                    ],
                    'message': 'Revert document [aclpolicies] to version [7a2448715e1ea8b97fc742cd51ec65703c9d0ef2]',
                    'email': 'curiefense@reblaze.com',
                    'author': 'Curiefense API'
                },
                {
                    'version': '6c439c1626b15011c8eac9117026e0bb4c9f3a1e',
                    'date': '2020-11-10T16:00:17+02:00',
                    'parents': [
                        '7a2448715e1ea8b97fc742cd51ec65703c9d0ef2'
                    ],
                    'message': 'Revert document [aclpolicies] to version [95c050c78a8ad2e66d946e216f3506a0f7b32278]',
                    'email': 'curiefense@reblaze.com',
                    'author': 'Curiefense API'
                },
                {
                    'version': '7a2448715e1ea8b97fc742cd51ec65703c9d0ef2',
                    'date': '2020-11-10T16:00:17+02:00',
                    'parents': [
                        '95c050c78a8ad2e66d946e216f3506a0f7b32278'
                    ],
                    'message': 'Revert document [aclpolicies] to version [b9cdba04b4ca00119bd148822ce0bc6a3761f017]',
                    'email': 'curiefense@reblaze.com',
                    'author': 'Curiefense API'
                },
                {
                    'version': '95c050c78a8ad2e66d946e216f3506a0f7b32278',
                    'date': '2020-11-10T16:00:16+02:00',
                    'parents': [
                        'b9cdba04b4ca00119bd148822ce0bc6a3761f017'
                    ],
                    'message': 'Revert document [aclpolicies] to version [134f8b24c3218e837243ae596a079cfc1ab671db]',
                    'email': 'curiefense@reblaze.com',
                    'author': 'Curiefense API'
                },
                {
                    'version': 'b9cdba04b4ca00119bd148822ce0bc6a3761f017',
                    'date': '2020-11-10T16:00:15+02:00',
                    'parents': [
                        '134f8b24c3218e837243ae596a079cfc1ab671db'
                    ],
                    'message': 'Revert document [aclpolicies] to version [fa54d53d087ad9d6b4b189188a1cfa68da718d79]',
                    'email': 'curiefense@reblaze.com',
                    'author': 'Curiefense API'
                },
                {
                    'version': '134f8b24c3218e837243ae596a079cfc1ab671db',
                    'date': '2020-11-10T16:00:15+02:00',
                    'parents': [
                        'fa54d53d087ad9d6b4b189188a1cfa68da718d79'
                    ],
                    'message': 'Revert document [aclpolicies] to version [0ff9c981493ee6d05720398425006fd8b0b1b856]',
                    'email': 'curiefense@reblaze.com',
                    'author': 'Curiefense API'
                },
                {
                    'version': 'fa54d53d087ad9d6b4b189188a1cfa68da718d79',
                    'date': '2020-11-10T16:00:14+02:00',
                    'parents': [
                        '0ff9c981493ee6d05720398425006fd8b0b1b856'
                    ],
                    'message': 'Revert document [aclpolicies] to version [efe3914cd898ef76bb0b9d12346090a1d112953c]',
                    'email': 'curiefense@reblaze.com',
                    'author': 'Curiefense API'
                },
                {
                    'version': '0ff9c981493ee6d05720398425006fd8b0b1b856',
                    'date': '2020-11-10T16:00:13+02:00',
                    'parents': [
                        'efe3914cd898ef76bb0b9d12346090a1d112953c'
                    ],
                    'message': 'Revert document [aclpolicies] to version [7b92d1c84d07ea51e9e5500ae86c327f554c7072]',
                    'email': 'curiefense@reblaze.com',
                    'author': 'Curiefense API'
                },
                {
                    'version': 'efe3914cd898ef76bb0b9d12346090a1d112953c',
                    'date': '2020-11-10T16:00:13+02:00',
                    'parents': [
                        '7b92d1c84d07ea51e9e5500ae86c327f554c7072'
                    ],
                    'message': 'Revert document [aclpolicies] to version [0eed6e6f205bccbff485527c3dab5ed5134beae9]',
                    'email': 'curiefense@reblaze.com',
                    'author': 'Curiefense API'
                },
                {
                    'version': '7b92d1c84d07ea51e9e5500ae86c327f554c7072',
                    'date': '2020-11-10T16:00:12+02:00',
                    'parents': [
                        '0eed6e6f205bccbff485527c3dab5ed5134beae9'
                    ],
                    'message': 'Revert document [aclpolicies] to version [7ebf09f77d8928025a889225233effc75de786d9]',
                    'email': 'curiefense@reblaze.com',
                    'author': 'Curiefense API'
                },
                {
                    'version': '0eed6e6f205bccbff485527c3dab5ed5134beae9',
                    'date': '2020-11-10T16:00:11+02:00',
                    'parents': [
                        '7ebf09f77d8928025a889225233effc75de786d9'
                    ],
                    'message': 'Revert document [aclpolicies] to version [8fc02c8a78fae8ac6bf7522222170b6541bae502]',
                    'email': 'curiefense@reblaze.com',
                    'author': 'Curiefense API'
                },
                {
                    'version': '7ebf09f77d8928025a889225233effc75de786d9',
                    'date': '2020-11-10T16:00:11+02:00',
                    'parents': [
                        '8fc02c8a78fae8ac6bf7522222170b6541bae502'
                    ],
                    'message': 'Revert document [aclpolicies] to version [1e350194f779279bbb9a3a80e89fa2bc45a06f16]',
                    'email': 'curiefense@reblaze.com',
                    'author': 'Curiefense API'
                },
                {
                    'version': '8fc02c8a78fae8ac6bf7522222170b6541bae502',
                    'date': '2020-11-10T16:00:10+02:00',
                    'parents': [
                        '1e350194f779279bbb9a3a80e89fa2bc45a06f16'
                    ],
                    'message': 'Revert document [aclpolicies] to version [99788d931b6018e8f940cc98c3c51aceb07fc77f]',
                    'email': 'curiefense@reblaze.com',
                    'author': 'Curiefense API'
                },
                {
                    'version': '1e350194f779279bbb9a3a80e89fa2bc45a06f16',
                    'date': '2020-11-10T16:00:09+02:00',
                    'parents': [
                        '99788d931b6018e8f940cc98c3c51aceb07fc77f'
                    ],
                    'message': 'Revert document [aclpolicies] to version [7dd9580c00bef1049ee9a531afb13db9ef3ee956]',
                    'email': 'curiefense@reblaze.com',
                    'author': 'Curiefense API'
                },
                {
                    'version': '99788d931b6018e8f940cc98c3c51aceb07fc77f',
                    'date': '2020-11-10T16:00:07+02:00',
                    'parents': [
                        '7dd9580c00bef1049ee9a531afb13db9ef3ee956'
                    ],
                    'message': 'Revert document [aclpolicies] to version [fc47a6cd9d7f254dd97875a04b87165cc484e075]',
                    'email': 'curiefense@reblaze.com',
                    'author': 'Curiefense API'
                },
                {
                    'version': '7dd9580c00bef1049ee9a531afb13db9ef3ee956',
                    'date': '2020-11-10T15:49:17+02:00',
                    'parents': [
                        'fc47a6cd9d7f254dd97875a04b87165cc484e075'
                    ],
                    'message': 'Update entry [__default__] of document [aclpolicies]',
                    'email': 'curiefense@reblaze.com',
                    'author': 'Curiefense API'
                },
                {
                    'version': 'fc47a6cd9d7f254dd97875a04b87165cc484e075',
                    'date': '2020-11-10T15:48:35+02:00',
                    'parents': [
                        '5aba4a5b9d6faea1896ee8965c7aa651f76af63c'
                    ],
                    'message': 'Update entry [__default__] of document [aclpolicies]',
                    'email': 'curiefense@reblaze.com',
                    'author': 'Curiefense API'
                },
                {
                    'version': '5aba4a5b9d6faea1896ee8965c7aa651f76af63c',
                    'date': '2020-11-10T15:48:31+02:00',
                    'parents': [
                        '277c5d7bd0e2eb4b9d2944f7eefdfadf37ba8581'
                    ],
                    'message': 'Update entry [__default__] of document [aclpolicies]',
                    'email': 'curiefense@reblaze.com',
                    'author': 'Curiefense API'
                },
                {
                    'version': '277c5d7bd0e2eb4b9d2944f7eefdfadf37ba8581',
                    'date': '2020-11-10T15:48:22+02:00',
                    'parents': [
                        '878b47deeddac94625fe7c759786f2df885ec541'
                    ],
                    'message': 'Update entry [__default__] of document [aclpolicies]',
                    'email': 'curiefense@reblaze.com',
                    'author': 'Curiefense API'
                },
                {
                    'version': '878b47deeddac94625fe7c759786f2df885ec541',
                    'date': '2020-11-10T15:48:05+02:00',
                    'parents': [
                        '93c180513fe7edeaf1c0ca69a67aa2a11374da4f'
                    ],
                    'message': 'Update entry [__default__] of document [aclpolicies]',
                    'email': 'curiefense@reblaze.com',
                    'author': 'Curiefense API'
                },
                {
                    'version': '93c180513fe7edeaf1c0ca69a67aa2a11374da4f',
                    'date': '2020-11-10T15:47:59+02:00',
                    'parents': [
                        '1662043d2a18d6ad2c9c94d6f826593ff5506354'
                    ],
                    'message': 'Update entry [__default__] of document [aclpolicies]',
                    'email': 'curiefense@reblaze.com',
                    'author': 'Curiefense API'
                },
                {
                    'version': '1662043d2a18d6ad2c9c94d6f826593ff5506354',
                    'date': '2020-11-08T21:31:41+01:00',
                    'parents': [
                        '16379cdf39501574b4a2f5a227b82a4454884b84'
                    ],
                    'message': 'Create config [master]\n',
                    'email': 'curiefense@reblaze.com',
                    'author': 'Curiefense API'
                }
            ]
        ]
        aclGitOldVersion = [
            {
                'id': '__default__',
                'name': 'default-acl',
                'allow': [],
                'allow_bot': [
                    'google'
                ],
                'deny_bot': [],
                'bypass': [
                    'internal'
                ],
                'deny': [
                    'tor'
                ],
                'force_deny': [
                    'china'
                ]
            },
            {
                'id': '5828321c37e0',
                'name': 'copy of default-acl',
                'allow': [],
                'allow_bot': [
                    'google'
                ],
                'deny_bot': [],
                'bypass': [
                    'internal'
                ],
                'deny': [
                    'tor'
                ],
                'force_deny': [
                    'china'
                ]
            }
        ]
        profilingListDocs = [
            {
                'id': 'xlbp148c',
                'name': 'API Discovery',
                'source': 'self-managed',
                'mdate': '2020-05-23T00:04:41',
                'notes': 'Tag API Requests',
                'active': true,
                'entries_relation': 'OR',
                'tags': [
                    'api'
                ],
                'entries': [
                    [
                        'headers',
                        [
                            'content-type',
                            '.*/(json|xml)'
                        ],
                        'content type'
                    ],
                    [
                        'headers',
                        [
                            'host',
                            '.?ap[ip]\\.'
                        ],
                        'app or api in domain name'
                    ],
                    [
                        'method',
                        '(POST|PUT|DELETE|PATCH)',
                        'Methods'
                    ],
                    [
                        'path',
                        '/api/',
                        'api path'
                    ],
                    [
                        'uri',
                        '/.+\\.json',
                        'URI JSON extention'
                    ]
                ]
            },
            {
                'id': '07656fbe',
                'name': 'devop internal demo',
                'source': 'self-managed',
                'mdate': '2020-05-23T00:04:41',
                'notes': 'this is my own list',
                'active': false,
                'entries_relation': 'OR',
                'tags': [
                    'internal',
                    'devops'
                ],
                'entries': [
                    [
                        'ip',
                        '12.34.56.78/32',
                        'testers'
                    ],
                    [
                        'ip',
                        '98.76.54.0/24',
                        'monitoring'
                    ],
                    [
                        'ip',
                        '!5.4.3.2/32',
                        'old monitoring'
                    ],
                    [
                        'path',
                        '/test/app/status',
                        'monitoring path'
                    ]
                ]
            }
        ]
        profilingListDocsLogs = [
            [
                {
                    'version': '1662043d2a18d6ad2c9c94d6f826593ff5506354',
                    'date': '2020-11-08T21:31:41+01:00',
                    'parents': [
                        '16379cdf39501574b4a2f5a227b82a4454884b84'
                    ],
                    'message': 'Create config [master]\n',
                    'email': 'curiefense@reblaze.com',
                    'author': 'Curiefense API'
                }
            ]
        ]
        urlMapsDocs = [
            {
                'id': '__default__',
                'name': 'default entry',
                'match': '__default__',
                'map': [
                    {
                        'name': 'default',
                        'match': '/',
                        'acl_profile': '5828321c37e0',
                        'acl_active': false,
                        'waf_profile': '__default__',
                        'waf_active': false,
                        'limit_ids': []
                    }
                ]
            }
        ]
        urlMapsDocsLogs = [
            [
                {
                    'version': '1662043d2a18d6ad2c9c94d6f826593ff5506354',
                    'date': '2020-11-08T21:31:41+01:00',
                    'parents': [
                        '16379cdf39501574b4a2f5a227b82a4454884b84'
                    ],
                    'message': 'Create config [master]\n',
                    'email': 'curiefense@reblaze.com',
                    'author': 'Curiefense API'
                }
            ]
        ]
        flowControlDocs = [
            {
                'exclude': [],
                'include': ['all'],
                'name': 'flow control',
                'key': [
                    {'headers': 'something'}
                ],
                'sequence': [
                    {
                        'method': 'GET',
                        'uri': '/login',
                        'cookies': {
                            'foo': 'bar'
                        },
                        'headers': {},
                        'args': {}
                    },
                    {
                        'method': 'POST',
                        'uri': '/login',
                        'cookies': {
                            'foo': 'bar'
                        },
                        'headers': {
                            'test': 'one'
                        },
                        'args': {}
                    }
                ],
                'action': {
                    'type': 'default',
                    'params': {}
                },
                'ttl': 60,
                'id': 'c03dabe4b9ca'
            }
        ]
        axios.get.mockImplementation((path) => {
            if (path === '/conf/api/v1/configs/') {
                return Promise.resolve({data: gitData})
            }
            const branch = wrapper.vm.selectedBranch
            const docID = wrapper.vm.selectedDocID
            if (path === `/conf/api/v1/configs/${branch}/d/aclpolicies/`) {
                return Promise.resolve({data: aclDocs})
            }
            if (path === `/conf/api/v1/configs/${branch}/d/aclpolicies/v/7f8a987c8e5e9db7c734ac8841c543d5bc5d9657/`) {
                return Promise.resolve({data: aclGitOldVersion})
            }
            if (path === `/conf/api/v1/configs/${branch}/d/aclpolicies/e/${docID}/v/`) {
                return Promise.resolve({data: aclDocsLogs[0]})
            }
            if (path === `/conf/api/v1/configs/${branch}/d/tagrules/`) {
                return Promise.resolve({data: profilingListDocs})
            }
            if (path === `/conf/api/v1/configs/${branch}/d/tagrules/e/${docID}/v/`) {
                return Promise.resolve({data: profilingListDocsLogs[0]})
            }
            if (path === `/conf/api/v1/configs/${branch}/d/urlmaps/`) {
                return Promise.resolve({data: urlMapsDocs})
            }
            if (path === `/conf/api/v1/configs/${branch}/d/urlmaps/e/${docID}/v/`) {
                return Promise.resolve({data: urlMapsDocsLogs[0]})
            }
            if (path === `/conf/api/v1/configs/${branch}/d/flowcontrol/`) {
                return Promise.resolve({data: flowControlDocs})
            }
            if (path === '/conf/api/v1/configs/master/v/') {
                return Promise.resolve({data: gitData[0].logs})
            }
            if (path === '/conf/api/v1/configs/zzz_branch/v/') {
                return Promise.resolve({data: gitData[1].logs})
            }
            return Promise.resolve({data: []})
        })
        mockRoute = {
            params: {
                branch: 'master',
                doc_type: 'aclpolicies',
                doc_id: '__default__'
            }
        }
        mockRouter = {
            push: jest.fn()
        }
        wrapper = shallowMount(DocumentEditor, {
            mocks: {
                $route: mockRoute,
                $router: mockRouter
            }
        })
        // allow all requests to finish
        setImmediate(() => {
            done()
        })
    })
    afterEach(() => {
        jest.clearAllMocks()
    })

    test('should have a git history component', () => {
        const gitHistory = wrapper.findComponent(GitHistory)
        expect(gitHistory).toBeTruthy()
    })

    test('should display correct amount of branches', () => {
        const gitBranches = wrapper.find('.git-branches')
        expect(gitBranches.text()).toContain('2 branches')
    })

    test('should display correct amount of commits', () => {
        const gitCommits = wrapper.find('.git-commits')
        expect(gitCommits.text()).toContain('10 commits')
    })

    test('should be able to switch branches through dropdown', (done) => {
        const branchSelection = wrapper.find('.branch-selection')
        branchSelection.trigger('click')
        const options = branchSelection.findAll('option')
        options.at(1).element.selected = true
        branchSelection.trigger('change')
        // allow all requests to finish
        setImmediate(() => {
            expect(branchSelection.element.selectedIndex).toEqual(1)
            done()
        })
    })

    test('should be able to switch doc types through dropdown', (done) => {
        const docTypeSelection = wrapper.find('.doc-type-selection')
        docTypeSelection.trigger('click')
        const options = docTypeSelection.findAll('option')
        options.at(2).element.selected = true
        docTypeSelection.trigger('change')
        // allow all requests to finish
        setImmediate(() => {
            expect(docTypeSelection.element.selectedIndex).toEqual(2)
            done()
        })
    })

    test('should be able to switch docs through dropdown', (done) => {
        // switch to profiling lists
        const docTypeSelection = wrapper.find('.doc-type-selection')
        docTypeSelection.trigger('click')
        const docTypeOptions = docTypeSelection.findAll('option')
        docTypeOptions.at(2).element.selected = true
        docTypeSelection.trigger('change')
        // allow all requests to finish
        setImmediate(() => {
            // switch to a different document
            const docSelection = wrapper.find('.doc-selection')
            docSelection.trigger('click')
            const options = docSelection.findAll('option')
            options.at(1).element.selected = true
            docSelection.trigger('change')
            // allow all requests to finish
            setImmediate(() => {
                expect(docSelection.element.selectedIndex).toEqual(1)
                done()
            })
        })
    })

    test('should refresh referenced IDs lists after saving url map entity', (done) => {
        // switch to url map entity type
        const docTypeSelection = wrapper.find('.doc-type-selection')
        docTypeSelection.trigger('click')
        const options = docTypeSelection.findAll('option')
        options.at(4).element.selected = true
        docTypeSelection.trigger('change')
        // allow all requests to finish
        setImmediate(async () => {
            const doc = wrapper.vm.selectedDoc
            doc.name = `${doc.name} changed`
            axios.put.mockImplementation(() => Promise.resolve())
            const getSpy = jest.spyOn(axios, 'get')
            const saveDocumentButton = wrapper.find('.save-document-button')
            saveDocumentButton.trigger('click')
            await Vue.nextTick()
            expect(getSpy).toHaveBeenCalledWith(`/conf/api/v1/configs/master/d/urlmaps/`)
            done()
        })
    })

    test('should be able to save document changes', async () => {
        const doc = wrapper.vm.selectedDoc
        doc.name = `${doc.name} changed`
        axios.put.mockImplementation(() => Promise.resolve())
        const putSpy = jest.spyOn(axios, 'put')
        const saveDocumentButton = wrapper.find('.save-document-button')
        saveDocumentButton.trigger('click')
        await Vue.nextTick()
        expect(putSpy).toHaveBeenCalledWith(`/conf/api/v1/configs/master/d/aclpolicies/e/${doc.id}/`, doc)
    })

    test('should be able to fork document', async () => {
        const originalDoc = wrapper.vm.selectedDoc
        const forkedDoc = {...originalDoc}
        forkedDoc.id = expect.any(String)
        forkedDoc.name = `copy of ${forkedDoc.name}`
        axios.post.mockImplementation(() => Promise.resolve())
        const postSpy = jest.spyOn(axios, 'post')
        await Vue.nextTick()
        const forkDocumentButton = wrapper.find('.fork-document-button')
        forkDocumentButton.trigger('click')
        await Vue.nextTick()
        expect(postSpy).toHaveBeenCalledWith(`/conf/api/v1/configs/master/d/aclpolicies/e/`, forkedDoc)
    })

    test('should be able to add a new document', async () => {
        const newDoc = DatasetsUtils.NewDocEntryFactory.aclpolicies()
        newDoc.id = expect.any(String)
        axios.post.mockImplementation(() => Promise.resolve())
        const postSpy = jest.spyOn(axios, 'post')
        const newDocumentButton = wrapper.find('.new-document-button')
        newDocumentButton.trigger('click')
        await Vue.nextTick()
        expect(postSpy).toHaveBeenCalledWith(`/conf/api/v1/configs/master/d/aclpolicies/e/`, newDoc)
    })

    test('should be able to delete a document', async () => {
        axios.delete.mockImplementation(() => Promise.resolve())
        const deleteSpy = jest.spyOn(axios, 'delete')
        // create new document so we can delete it
        const newDocumentButton = wrapper.find('.new-document-button')
        newDocumentButton.trigger('click')
        await Vue.nextTick()
        const docID = wrapper.vm.selectedDocID
        const deleteDocumentButton = wrapper.find('.delete-document-button')
        deleteDocumentButton.trigger('click')
        await Vue.nextTick()
        expect(deleteSpy).toHaveBeenCalledWith(`/conf/api/v1/configs/master/d/aclpolicies/e/${docID}/`)
    })

    test('should not be able to delete a document if its id is __default__', async () => {
        axios.delete.mockImplementation(() => Promise.resolve())
        wrapper.vm.selectedDocID = '__default__'
        await Vue.nextTick()
        const deleteSpy = jest.spyOn(axios, 'delete')
        const deleteDocumentButton = wrapper.find('.delete-document-button')
        deleteDocumentButton.trigger('click')
        await Vue.nextTick()
        expect(deleteSpy).not.toHaveBeenCalled()
    })

    test('should not be able to delete a document if it is referenced by a url map', async () => {
        // switch to a different document
        const docSelection = wrapper.find('.doc-selection')
        docSelection.trigger('click')
        const options = docSelection.findAll('option')
        options.at(1).element.selected = true
        docSelection.trigger('change')
        await Vue.nextTick()
        axios.delete.mockImplementation(() => Promise.resolve())
        wrapper.vm.selectedDoc.id = '__default__'
        const deleteSpy = jest.spyOn(axios, 'delete')
        const deleteDocumentButton = wrapper.find('.delete-document-button')
        deleteDocumentButton.trigger('click')
        await Vue.nextTick()
        expect(deleteSpy).not.toHaveBeenCalled()
    })

    test('should send API request to restore to the correct version', async () => {
        const wantedVersion = {
            version: '7f8a987c8e5e9db7c734ac8841c543d5bc5d9657'
        }
        let putSpy
        axios.put.mockImplementation(() => Promise.resolve())
        putSpy = jest.spyOn(axios, 'put')
        const gitHistory = wrapper.findComponent(GitHistory)
        gitHistory.vm.$emit('restore-version', wantedVersion)
        await Vue.nextTick()
        expect(putSpy).toHaveBeenCalledWith(`/conf/api/v1/configs/master/d/aclpolicies/v/${wantedVersion.version}/revert/`)
    })

    test('should log message when receiving no configs from the server', (done) => {
        const originalLog = console.log
        let consoleOutput = []
        const mockedLog = output => consoleOutput.push(output)
        consoleOutput = []
        console.log = mockedLog
        axios.get.mockImplementation((path) => {
            if (path === '/conf/api/v1/configs/') {
                return Promise.reject()
            }
            return Promise.resolve({data: {}})
        })
        wrapper = shallowMount(DocumentEditor, {
            mocks: {
                $route: mockRoute,
                $router: mockRouter
            }
        })
        // allow all requests to finish
        setImmediate(() => {
            expect(consoleOutput).toContain(`Error while attempting to get configs`)
            console.log = originalLog
            done()
        })
    })

    test('should attempt to download document when download button is clicked', async () => {
        const wantedFileName = 'aclpolicies'
        const wantedFileType = 'json'
        const wantedFileData = aclDocs
        const downloadFileSpy = jest.spyOn(Utils, 'downloadFile')
        // force update because downloadFile is mocked after it is read to to be used as event handler
        await wrapper.vm.$forceUpdate()
        await Vue.nextTick()
        const downloadDocButton = wrapper.find('.download-doc-button')
        downloadDocButton.trigger('click')
        await Vue.nextTick()
        expect(downloadFileSpy).toHaveBeenCalledWith(wantedFileName, wantedFileType, wantedFileData)
    })

    describe('no data', () => {
        test('should display correct message when there is no branch data', (done) => {
            axios.get.mockImplementation((path) => {
                if (path === '/conf/api/v1/configs/') {
                    return Promise.resolve({data: []})
                }
                return Promise.resolve({data: []})
            })
            wrapper = shallowMount(DocumentEditor, {
                mocks: {
                    $route: mockRoute,
                    $router: mockRouter
                }
            })
            // allow all requests to finish
            setImmediate(() => {
                const noDataMessage = wrapper.find('.no-data-message')
                expect(noDataMessage.element).toBeDefined()
                expect(noDataMessage.text().toLowerCase()).toContain('no data found!')
                expect(noDataMessage.text().toLowerCase()).toContain('missing branch.')
                done()
            })
        })

        test('should display correct message when there is no doc type data', (done) => {
            // it is not possible to get to this state from the UI, but we protect from it anyway
            wrapper.vm.selectedDocType = null
            // allow all requests to finish
            setImmediate(() => {
                const noDataMessage = wrapper.find('.no-data-message')
                expect(noDataMessage.element).toBeDefined()
                expect(noDataMessage.text().toLowerCase()).toContain('no data found!')
                expect(noDataMessage.text().toLowerCase()).toContain('missing document type.')
                done()
            })
        })

        test('should display correct message when there is no doc data', (done) => {
            axios.get.mockImplementation((path) => {
                if (path === '/conf/api/v1/configs/') {
                    return Promise.resolve({data: gitData})
                }
                return Promise.resolve({data: []})
            })
            wrapper = shallowMount(DocumentEditor, {
                mocks: {
                    $route: mockRoute,
                    $router: mockRouter
                }
            })
            // allow all requests to finish
            setImmediate(() => {
                const noDataMessage = wrapper.find('.no-data-message')
                expect(noDataMessage.element).toBeDefined()
                expect(noDataMessage.text().toLowerCase()).toContain('no data found!')
                expect(noDataMessage.text().toLowerCase()).toContain('missing document.')
                done()
            })
        })
    })

    describe('loading indicator', () => {
        test('should display loading indicator when branch not loaded', async () => {
            axios.get.mockImplementation((path) => {
                if (path === '/conf/api/v1/configs/') {
                    return new Promise(() => {
                    })
                }
                return Promise.resolve({data: []})
            })
            wrapper = shallowMount(DocumentEditor, {
                mocks: {
                    $route: mockRoute,
                    $router: mockRouter
                }
            })
            await Vue.nextTick()
            const docLoadingIndicator = wrapper.find('.document-loading')
            expect(docLoadingIndicator.element).toBeDefined()
        })

        test('should display loading indicator when doc not loaded', async () => {
            axios.get.mockImplementation((path) => {
                if (path === '/conf/api/v1/configs/') {
                    return Promise.resolve({data: gitData})
                }
                const branch = wrapper.vm.selectedBranch
                if (path === `/conf/api/v1/configs/${branch}/d/aclpolicies/`) {
                    return new Promise(() => {
                    })
                }
                return Promise.resolve({data: []})
            })
            wrapper = shallowMount(DocumentEditor, {
                mocks: {
                    $route: mockRoute,
                    $router: mockRouter
                }
            })
            await Vue.nextTick()
            const docLoadingIndicator = wrapper.find('.document-loading')
            expect(docLoadingIndicator.element).toBeDefined()
        })

        test('should display loading indicator when saving document changes', async () => {
            axios.put.mockImplementation(() => new Promise(() => {
            }))
            const saveDocumentButton = wrapper.find('.save-document-button')
            saveDocumentButton.trigger('click')
            await Vue.nextTick()
            expect(saveDocumentButton.element.classList).toContain('is-loading')
        })

        test('should display loading indicator when forking document', async () => {
            axios.post.mockImplementation(() => new Promise(() => {
            }))
            const forkDocumentButton = wrapper.find('.fork-document-button')
            forkDocumentButton.trigger('click')
            await Vue.nextTick()
            expect(forkDocumentButton.element.classList).toContain('is-loading')
        })

        test('should display loading indicator when adding a new document', async () => {
            axios.post.mockImplementation(() => new Promise(() => {
            }))
            const newDocumentButton = wrapper.find('.new-document-button')
            newDocumentButton.trigger('click')
            await Vue.nextTick()
            expect(newDocumentButton.element.classList).toContain('is-loading')
        })

        test('should display loading indicator when deleting a document', async () => {
            axios.delete.mockImplementation(() => new Promise(() => {
            }))
            // create new document so we can delete it
            const newDocumentButton = wrapper.find('.new-document-button')
            newDocumentButton.trigger('click')
            await Vue.nextTick()
            const deleteDocumentButton = wrapper.find('.delete-document-button')
            deleteDocumentButton.trigger('click')
            await Vue.nextTick()
            expect(deleteDocumentButton.element.classList).toContain('is-loading')
        })
    })
})
