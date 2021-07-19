import DocumentEditor from '@/views/DocumentEditor.vue'
import GitHistory from '@/components/GitHistory.vue'
import DatasetsUtils from '@/assets/DatasetsUtils'
import Utils from '@/assets/Utils'
import {afterEach, beforeEach, describe, expect, jest, test} from '@jest/globals'
import {shallowMount, Wrapper} from '@vue/test-utils'
import Vue from 'vue'
import axios from 'axios'
import _ from 'lodash'
import {ACLProfile, Branch, Commit, ContentFilterProfile, Document, FlowControlPolicy, GlobalFilter, RateLimit, SecurityPolicy} from '@/types'

jest.mock('axios')

describe('DocumentEditor.vue', () => {
  let wrapper: Wrapper<Vue>
  let mockRoute: any
  let mockRouter: any
  let gitData: Branch[]
  let aclDocs: ACLProfile[]
  let aclDocsLogs: Commit[][]
  let aclGitOldVersion: ACLProfile[]
  let profilingListDocs: GlobalFilter[]
  let profilingListDocsLogs: Commit[][]
  let securityPoliciesDocs: SecurityPolicy[]
  let securityPoliciesDocsLogs: Commit[][]
  let flowControlPolicyDocs: FlowControlPolicy[]
  let contentFilterDocs: ContentFilterProfile[]
  let rateLimitsDocs: RateLimit[]
  beforeEach((done) => {
    gitData = [
      {
        'id': 'master',
        'description': 'Update entry [__default__] of document [aclprofiles]',
        'date': '2020-11-10T15:49:17+02:00',
        'logs': [
          {
            'version': '7dd9580c00bef1049ee9a531afb13db9ef3ee956',
            'date': '2020-11-10T15:49:17+02:00',
            'parents': [
              'fc47a6cd9d7f254dd97875a04b87165cc484e075',
            ],
            'message': 'Update entry [__default__] of document [aclprofiles]',
            'email': 'curiefense@reblaze.com',
            'author': 'Curiefense API',
          },
          {
            'version': 'fc47a6cd9d7f254dd97875a04b87165cc484e075',
            'date': '2020-11-10T15:48:35+02:00',
            'parents': [
              '5aba4a5b9d6faea1896ee8965c7aa651f76af63c',
            ],
            'message': 'Update entry [__default__] of document [aclprofiles]',
            'email': 'curiefense@reblaze.com',
            'author': 'Curiefense API',
          },
          {
            'version': '5aba4a5b9d6faea1896ee8965c7aa651f76af63c',
            'date': '2020-11-10T15:48:31+02:00',
            'parents': [
              '277c5d7bd0e2eb4b9d2944f7eefdfadf37ba8581',
            ],
            'message': 'Update entry [__default__] of document [aclprofiles]',
            'email': 'curiefense@reblaze.com',
            'author': 'Curiefense API',
          },
          {
            'version': '277c5d7bd0e2eb4b9d2944f7eefdfadf37ba8581',
            'date': '2020-11-10T15:48:22+02:00',
            'parents': [
              '878b47deeddac94625fe7c759786f2df885ec541',
            ],
            'message': 'Update entry [__default__] of document [aclprofiles]',
            'email': 'curiefense@reblaze.com',
            'author': 'Curiefense API',
          },
          {
            'version': '878b47deeddac94625fe7c759786f2df885ec541',
            'date': '2020-11-10T15:48:05+02:00',
            'parents': [
              '93c180513fe7edeaf1c0ca69a67aa2a11374da4f',
            ],
            'message': 'Update entry [__default__] of document [aclprofiles]',
            'email': 'curiefense@reblaze.com',
            'author': 'Curiefense API',
          },
          {
            'version': '93c180513fe7edeaf1c0ca69a67aa2a11374da4f',
            'date': '2020-11-10T15:47:59+02:00',
            'parents': [
              '1662043d2a18d6ad2c9c94d6f826593ff5506354',
            ],
            'message': 'Update entry [__default__] of document [aclprofiles]',
            'email': 'curiefense@reblaze.com',
            'author': 'Curiefense API',
          },
          {
            'version': '1662043d2a18d6ad2c9c94d6f826593ff5506354',
            'date': '2020-11-08T21:31:41+01:00',
            'parents': [
              '16379cdf39501574b4a2f5a227b82a4454884b84',
            ],
            'message': 'Create config [master]\n',
            'email': 'curiefense@reblaze.com',
            'author': 'Curiefense API',
          },
          {
            'version': '16379cdf39501574b4a2f5a227b82a4454884b84',
            'date': '2020-08-27T16:19:06+00:00',
            'parents': [
              'a34f979217215060861b58b3f270e82580c20efb',
            ],
            'message': 'Initial empty config',
            'email': 'curiefense@reblaze.com',
            'author': 'Curiefense API',
          },
          {
            'version': 'a34f979217215060861b58b3f270e82580c20efb',
            'date': '2020-08-27T16:19:06+00:00',
            'parents': [],
            'message': 'Initial empty content',
            'email': 'curiefense@reblaze.com',
            'author': 'Curiefense API',
          },
        ],
        'version': '7dd9580c00bef1049ee9a531afb13db9ef3ee956',
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
            'author': 'Curiefense API',
          },
        ],
        'version': 'a34f979217215060861b58b3f270e82580c20efb',
      },
    ]
    aclDocs = [
      {
        'id': '__default__',
        'name': 'default-acl',
        'allow': [],
        'allow_bot': [
          'google',
        ],
        'deny_bot': [],
        'passthrough': [
          'internal',
        ],
        'deny': [
          'tor',
        ],
        'force_deny': [
          'china',
        ],
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
        'passthrough': [
          'devops',
        ],
        'deny': [
          'tor',
        ],
        'force_deny': [
          'iran',
        ],
      },
    ]
    aclDocsLogs = [
      [
        {
          'version': '7f8a987c8e5e9db7c734ac8841c543d5bc5d9657',
          'date': '2020-11-12T17:23:11+02:00',
          'parents': [
            '82d8f29096af1db07dbf7e1cff581fdf6e1a7440',
          ],
          'message': 'Revert to version [7a24bd37e93e812fa5173c4b2fb0068ad8e4ffdd]',
          'email': 'curiefense@reblaze.com',
          'author': 'Curiefense API',
        },
        {
          'version': '82d8f29096af1db07dbf7e1cff581fdf6e1a7440',
          'date': '2020-11-12T17:22:56+02:00',
          'parents': [
            '7a24bd37e93e812fa5173c4b2fb0068ad8e4ffdd',
          ],
          'message': 'Update entry [__default__] of document [aclprofiles]',
          'email': 'curiefense@reblaze.com',
          'author': 'Curiefense API',
        },
        {
          'version': '7a24bd37e93e812fa5173c4b2fb0068ad8e4ffdd',
          'date': '2020-11-12T17:22:53+02:00',
          'parents': [
            '886baf66ddd032744ac34b848c8412386a160fb3',
          ],
          'message': 'Update entry [__default__] of document [aclprofiles]',
          'email': 'curiefense@reblaze.com',
          'author': 'Curiefense API',
        },
        {
          'version': '886baf66ddd032744ac34b848c8412386a160fb3',
          'date': '2020-11-12T17:22:51+02:00',
          'parents': [
            'af98cb28fc4db3a76c3a51d697b6037e8695dd7b',
          ],
          'message': 'Update entry [__default__] of document [aclprofiles]',
          'email': 'curiefense@reblaze.com',
          'author': 'Curiefense API',
        },
        {
          'version': 'af98cb28fc4db3a76c3a51d697b6037e8695dd7b',
          'date': '2020-11-12T17:22:50+02:00',
          'parents': [
            'cda70058b632405600db1fbc5cf8dfd90514ec30',
          ],
          'message': 'Update entry [__default__] of document [aclprofiles]',
          'email': 'curiefense@reblaze.com',
          'author': 'Curiefense API',
        },
        {
          'version': 'e57349cabbb31dd4f07945a42c2ab91671a5a7b1',
          'date': '2020-11-10T16:00:18+02:00',
          'parents': [
            '78d2193cdaa2818734894ab5bbb85cf932d4f217',
          ],
          'message': 'Revert document [aclprofiles] to version [6c439c1626b15011c8eac9117026e0bb4c9f3a1e]',
          'email': 'curiefense@reblaze.com',
          'author': 'Curiefense API',
        },
        {
          'version': '78d2193cdaa2818734894ab5bbb85cf932d4f217',
          'date': '2020-11-10T16:00:18+02:00',
          'parents': [
            '6c439c1626b15011c8eac9117026e0bb4c9f3a1e',
          ],
          'message': 'Revert document [aclprofiles] to version [7a2448715e1ea8b97fc742cd51ec65703c9d0ef2]',
          'email': 'curiefense@reblaze.com',
          'author': 'Curiefense API',
        },
        {
          'version': '6c439c1626b15011c8eac9117026e0bb4c9f3a1e',
          'date': '2020-11-10T16:00:17+02:00',
          'parents': [
            '7a2448715e1ea8b97fc742cd51ec65703c9d0ef2',
          ],
          'message': 'Revert document [aclprofiles] to version [95c050c78a8ad2e66d946e216f3506a0f7b32278]',
          'email': 'curiefense@reblaze.com',
          'author': 'Curiefense API',
        },
        {
          'version': '7a2448715e1ea8b97fc742cd51ec65703c9d0ef2',
          'date': '2020-11-10T16:00:17+02:00',
          'parents': [
            '95c050c78a8ad2e66d946e216f3506a0f7b32278',
          ],
          'message': 'Revert document [aclprofiles] to version [b9cdba04b4ca00119bd148822ce0bc6a3761f017]',
          'email': 'curiefense@reblaze.com',
          'author': 'Curiefense API',
        },
        {
          'version': '95c050c78a8ad2e66d946e216f3506a0f7b32278',
          'date': '2020-11-10T16:00:16+02:00',
          'parents': [
            'b9cdba04b4ca00119bd148822ce0bc6a3761f017',
          ],
          'message': 'Revert document [aclprofiles] to version [134f8b24c3218e837243ae596a079cfc1ab671db]',
          'email': 'curiefense@reblaze.com',
          'author': 'Curiefense API',
        },
        {
          'version': 'b9cdba04b4ca00119bd148822ce0bc6a3761f017',
          'date': '2020-11-10T16:00:15+02:00',
          'parents': [
            '134f8b24c3218e837243ae596a079cfc1ab671db',
          ],
          'message': 'Revert document [aclprofiles] to version [fa54d53d087ad9d6b4b189188a1cfa68da718d79]',
          'email': 'curiefense@reblaze.com',
          'author': 'Curiefense API',
        },
        {
          'version': '134f8b24c3218e837243ae596a079cfc1ab671db',
          'date': '2020-11-10T16:00:15+02:00',
          'parents': [
            'fa54d53d087ad9d6b4b189188a1cfa68da718d79',
          ],
          'message': 'Revert document [aclprofiles] to version [0ff9c981493ee6d05720398425006fd8b0b1b856]',
          'email': 'curiefense@reblaze.com',
          'author': 'Curiefense API',
        },
        {
          'version': 'fa54d53d087ad9d6b4b189188a1cfa68da718d79',
          'date': '2020-11-10T16:00:14+02:00',
          'parents': [
            '0ff9c981493ee6d05720398425006fd8b0b1b856',
          ],
          'message': 'Revert document [aclprofiles] to version [efe3914cd898ef76bb0b9d12346090a1d112953c]',
          'email': 'curiefense@reblaze.com',
          'author': 'Curiefense API',
        },
        {
          'version': '0ff9c981493ee6d05720398425006fd8b0b1b856',
          'date': '2020-11-10T16:00:13+02:00',
          'parents': [
            'efe3914cd898ef76bb0b9d12346090a1d112953c',
          ],
          'message': 'Revert document [aclprofiles] to version [7b92d1c84d07ea51e9e5500ae86c327f554c7072]',
          'email': 'curiefense@reblaze.com',
          'author': 'Curiefense API',
        },
        {
          'version': 'efe3914cd898ef76bb0b9d12346090a1d112953c',
          'date': '2020-11-10T16:00:13+02:00',
          'parents': [
            '7b92d1c84d07ea51e9e5500ae86c327f554c7072',
          ],
          'message': 'Revert document [aclprofiles] to version [0eed6e6f205bccbff485527c3dab5ed5134beae9]',
          'email': 'curiefense@reblaze.com',
          'author': 'Curiefense API',
        },
        {
          'version': '7b92d1c84d07ea51e9e5500ae86c327f554c7072',
          'date': '2020-11-10T16:00:12+02:00',
          'parents': [
            '0eed6e6f205bccbff485527c3dab5ed5134beae9',
          ],
          'message': 'Revert document [aclprofiles] to version [7ebf09f77d8928025a889225233effc75de786d9]',
          'email': 'curiefense@reblaze.com',
          'author': 'Curiefense API',
        },
        {
          'version': '0eed6e6f205bccbff485527c3dab5ed5134beae9',
          'date': '2020-11-10T16:00:11+02:00',
          'parents': [
            '7ebf09f77d8928025a889225233effc75de786d9',
          ],
          'message': 'Revert document [aclprofiles] to version [8fc02c8a78fae8ac6bf7522222170b6541bae502]',
          'email': 'curiefense@reblaze.com',
          'author': 'Curiefense API',
        },
        {
          'version': '7ebf09f77d8928025a889225233effc75de786d9',
          'date': '2020-11-10T16:00:11+02:00',
          'parents': [
            '8fc02c8a78fae8ac6bf7522222170b6541bae502',
          ],
          'message': 'Revert document [aclprofiles] to version [1e350194f779279bbb9a3a80e89fa2bc45a06f16]',
          'email': 'curiefense@reblaze.com',
          'author': 'Curiefense API',
        },
        {
          'version': '8fc02c8a78fae8ac6bf7522222170b6541bae502',
          'date': '2020-11-10T16:00:10+02:00',
          'parents': [
            '1e350194f779279bbb9a3a80e89fa2bc45a06f16',
          ],
          'message': 'Revert document [aclprofiles] to version [99788d931b6018e8f940cc98c3c51aceb07fc77f]',
          'email': 'curiefense@reblaze.com',
          'author': 'Curiefense API',
        },
        {
          'version': '1e350194f779279bbb9a3a80e89fa2bc45a06f16',
          'date': '2020-11-10T16:00:09+02:00',
          'parents': [
            '99788d931b6018e8f940cc98c3c51aceb07fc77f',
          ],
          'message': 'Revert document [aclprofiles] to version [7dd9580c00bef1049ee9a531afb13db9ef3ee956]',
          'email': 'curiefense@reblaze.com',
          'author': 'Curiefense API',
        },
        {
          'version': '99788d931b6018e8f940cc98c3c51aceb07fc77f',
          'date': '2020-11-10T16:00:07+02:00',
          'parents': [
            '7dd9580c00bef1049ee9a531afb13db9ef3ee956',
          ],
          'message': 'Revert document [aclprofiles] to version [fc47a6cd9d7f254dd97875a04b87165cc484e075]',
          'email': 'curiefense@reblaze.com',
          'author': 'Curiefense API',
        },
        {
          'version': '7dd9580c00bef1049ee9a531afb13db9ef3ee956',
          'date': '2020-11-10T15:49:17+02:00',
          'parents': [
            'fc47a6cd9d7f254dd97875a04b87165cc484e075',
          ],
          'message': 'Update entry [__default__] of document [aclprofiles]',
          'email': 'curiefense@reblaze.com',
          'author': 'Curiefense API',
        },
        {
          'version': 'fc47a6cd9d7f254dd97875a04b87165cc484e075',
          'date': '2020-11-10T15:48:35+02:00',
          'parents': [
            '5aba4a5b9d6faea1896ee8965c7aa651f76af63c',
          ],
          'message': 'Update entry [__default__] of document [aclprofiles]',
          'email': 'curiefense@reblaze.com',
          'author': 'Curiefense API',
        },
        {
          'version': '5aba4a5b9d6faea1896ee8965c7aa651f76af63c',
          'date': '2020-11-10T15:48:31+02:00',
          'parents': [
            '277c5d7bd0e2eb4b9d2944f7eefdfadf37ba8581',
          ],
          'message': 'Update entry [__default__] of document [aclprofiles]',
          'email': 'curiefense@reblaze.com',
          'author': 'Curiefense API',
        },
        {
          'version': '277c5d7bd0e2eb4b9d2944f7eefdfadf37ba8581',
          'date': '2020-11-10T15:48:22+02:00',
          'parents': [
            '878b47deeddac94625fe7c759786f2df885ec541',
          ],
          'message': 'Update entry [__default__] of document [aclprofiles]',
          'email': 'curiefense@reblaze.com',
          'author': 'Curiefense API',
        },
        {
          'version': '878b47deeddac94625fe7c759786f2df885ec541',
          'date': '2020-11-10T15:48:05+02:00',
          'parents': [
            '93c180513fe7edeaf1c0ca69a67aa2a11374da4f',
          ],
          'message': 'Update entry [__default__] of document [aclprofiles]',
          'email': 'curiefense@reblaze.com',
          'author': 'Curiefense API',
        },
        {
          'version': '93c180513fe7edeaf1c0ca69a67aa2a11374da4f',
          'date': '2020-11-10T15:47:59+02:00',
          'parents': [
            '1662043d2a18d6ad2c9c94d6f826593ff5506354',
          ],
          'message': 'Update entry [__default__] of document [aclprofiles]',
          'email': 'curiefense@reblaze.com',
          'author': 'Curiefense API',
        },
        {
          'version': '1662043d2a18d6ad2c9c94d6f826593ff5506354',
          'date': '2020-11-08T21:31:41+01:00',
          'parents': [
            '16379cdf39501574b4a2f5a227b82a4454884b84',
          ],
          'message': 'Create config [master]\n',
          'email': 'curiefense@reblaze.com',
          'author': 'Curiefense API',
        },
      ],
      [
        {
          'version': '93c180513fe7edeaf1c0ca69a67aa2a11374da4f',
          'date': '2020-11-10T15:47:59+02:00',
          'parents': [
            '1662043d2a18d6ad2c9c94d6f826593ff5506354',
          ],
          'message': 'Update entry [__default__] of document [aclprofiles]',
          'email': 'curiefense@reblaze.com',
          'author': 'Curiefense API',
        },
        {
          'version': '1662043d2a18d6ad2c9c94d6f826593ff5506354',
          'date': '2020-11-08T21:31:41+01:00',
          'parents': [
            '16379cdf39501574b4a2f5a227b82a4454884b84',
          ],
          'message': 'Create config [master]\n',
          'email': 'curiefense@reblaze.com',
          'author': 'Curiefense API',
        },
      ],
    ]
    aclGitOldVersion = [
      {
        'id': '__default__',
        'name': 'default-acl',
        'allow': [],
        'allow_bot': [
          'google',
        ],
        'deny_bot': [],
        'passthrough': [
          'internal',
        ],
        'deny': [
          'tor',
        ],
        'force_deny': [
          'china',
        ],
      },
      {
        'id': '5828321c37e0',
        'name': 'copy of default-acl',
        'allow': [],
        'allow_bot': [
          'google',
        ],
        'deny_bot': [],
        'passthrough': [
          'internal',
        ],
        'deny': [
          'tor',
        ],
        'force_deny': [
          'china',
        ],
      },
    ]
    profilingListDocs = [
      {
        'id': 'xlbp148c',
        'name': 'API Discovery',
        'source': 'self-managed',
        'mdate': '2020-05-23T00:04:41',
        'description': 'Tag API Requests',
        'active': true,
        'tags': ['api'],
        'action': {
          'type': 'monitor',
          'params': {},
        },
        'rule': {
          'relation': 'OR',
          'sections': [
            {'relation': 'OR', 'entries': [['ip', '1.1.1.1', null]]},
            {'relation': 'OR', 'entries': [['ip', '2.2.2.2', null]]},
            {'relation': 'OR', 'entries': [['headers', ['headerrr', 'valueeee'], 'anooo']]}],
        },
      }, {
        'id': '07656fbe',
        'name': 'devop internal demo',
        'source': 'self-managed',
        'mdate': '2020-05-23T00:04:41',
        'description': 'this is my own list',
        'active': false,
        'tags': ['internal', 'devops'],
        'action': {
          'type': 'monitor',
          'params': {},
        },
        'rule': {
          'relation': 'OR',
          'sections': [
            {'relation': 'OR', 'entries': [['ip', '1.1.1.1', null]]},
            {'relation': 'OR', 'entries': [['ip', '2.2.2.2', null]]},
            {'relation': 'OR', 'entries': [['headers', ['headerrr', 'valueeee'], 'anooo']]}],
        },
      },
    ]
    profilingListDocsLogs = [
      [
        {
          'version': '1662043d2a18d6ad2c9c94d6f826593ff5506354',
          'date': '2020-11-08T21:31:41+01:00',
          'parents': [
            '16379cdf39501574b4a2f5a227b82a4454884b84',
          ],
          'message': 'Create config [master]\n',
          'email': 'curiefense@reblaze.com',
          'author': 'Curiefense API',
        },
      ],
    ]
    securityPoliciesDocs = [
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
            'content_filter_profile': '009e846e819e',
            'content_filter_active': false,
            'limit_ids': [],
          },
        ],
      },
    ]
    securityPoliciesDocsLogs = [
      [
        {
          'version': '1662043d2a18d6ad2c9c94d6f826593ff5506354',
          'date': '2020-11-08T21:31:41+01:00',
          'parents': [
            '16379cdf39501574b4a2f5a227b82a4454884b84',
          ],
          'message': 'Create config [master]\n',
          'email': 'curiefense@reblaze.com',
          'author': 'Curiefense API',
        },
      ],
    ]
    flowControlPolicyDocs = [
      {
        'active': true,
        'description': '',
        'exclude': [],
        'include': ['all'],
        'name': 'flow control policy',
        'key': [
          {'headers': 'something'},
        ],
        'sequence': [
          {
            'method': 'GET',
            'uri': '/login',
            'cookies': {
              'foo': 'bar',
            },
            'headers': {},
            'args': {},
          },
          {
            'method': 'POST',
            'uri': '/login',
            'cookies': {
              'foo': 'bar',
            },
            'headers': {
              'test': 'one',
            },
            'args': {},
          },
        ],
        'action': {
          'type': 'default',
          'params': {},
        },
        'timeframe': 60,
        'id': 'c03dabe4b9ca',
      },
    ]
    contentFilterDocs = [{
      'id': '009e846e819e',
      'name': 'content filter',
      'ignore_alphanum': true,
      'max_header_length': 1024,
      'max_cookie_length': 2048,
      'max_arg_length': 1536,
      'max_headers_count': 36,
      'max_cookies_count': 42,
      'max_args_count': 512,
      'min_headers_risk': 1,
      'min_cookies_risk': 1,
      'min_args_risk': 1,
      'args': {'names': [], 'regex': []},
      'headers': {'names': [], 'regex': []},
      'cookies': {'names': [], 'regex': []},
    }]
    rateLimitsDocs = [{
      'id': 'f971e92459e2',
      'name': 'Rate Limit Example Rule 5/60',
      'description': '5 requests per minute',
      'timeframe': '60',
      'thresholds': [
        {
          'limit': '5',
          'action': {'type': 'default', 'params': {'action': {'type': 'default', 'params': {}}}},
        },
      ],
      'include': ['badpeople'],
      'exclude': ['goodpeople'],
      'key': [{'attrs': 'ip'}],
      'pairwith': {'self': 'self'},
    }]
    jest.spyOn(axios.CancelToken, 'source').mockImplementation(() => {
      return {
        token: null,
        cancel: () => {
        },
      }
    })
    jest.spyOn(axios, 'get').mockImplementation((path, config) => {
      if (path === '/conf/api/v2/configs/') {
        return Promise.resolve({data: gitData})
      }
      const branch = (wrapper.vm as any).selectedBranch
      const docID = (wrapper.vm as any).selectedDocID
      if (path === `/conf/api/v2/configs/${branch}/d/aclprofiles/`) {
        if (config && config.headers && config.headers['x-fields'] === 'id, name') {
          return Promise.resolve({data: _.map(aclDocs, (i) => _.pick(i, 'id', 'name'))})
        }
        return Promise.resolve({data: aclDocs})
      }
      if (path === `/conf/api/v2/configs/${branch}/d/aclprofiles/e/__default__/`) {
        return Promise.resolve({data: aclDocs[0]})
      }
      if (path === `/conf/api/v2/configs/${branch}/d/aclprofiles/e/5828321c37e0/`) {
        return Promise.resolve({data: aclDocs[1]})
      }
      if (path === `/conf/api/v2/configs/${branch}/d/aclprofiles/v/7f8a987c8e5e9db7c734ac8841c543d5bc5d9657/`) {
        return Promise.resolve({data: aclGitOldVersion})
      }
      if (path === `/conf/api/v2/configs/master/d/aclprofiles/e/__default__/v/`) {
        return Promise.resolve({data: aclDocsLogs[0]})
      }
      if (path === `/conf/api/v2/configs/zzz_branch/d/aclprofiles/e/__default__/v/`) {
        return Promise.resolve({data: aclDocsLogs[1]})
      }
      if (path === `/conf/api/v2/configs/${branch}/d/aclprofiles/e/5828321c37e0/v/`) {
        return Promise.resolve({data: aclDocsLogs[1]})
      }
      if (path === `/conf/api/v2/configs/${branch}/d/globalfilters/`) {
        if (config && config.headers && config.headers['x-fields'] === 'id, name') {
          return Promise.resolve({data: _.map(profilingListDocs, (i) => _.pick(i, 'id', 'name'))})
        }
        return Promise.resolve({data: profilingListDocs})
      }
      if (path === `/conf/api/v2/configs/${branch}/d/globalfilters/e/xlbp148c/`) {
        return Promise.resolve({data: profilingListDocs[0]})
      }
      if (path === `/conf/api/v2/configs/${branch}/d/globalfilters/e/07656fbe/`) {
        return Promise.resolve({data: profilingListDocs[1]})
      }
      if (path === `/conf/api/v2/configs/${branch}/d/globalfilters/e/${docID}/v/`) {
        return Promise.resolve({data: profilingListDocsLogs[0]})
      }
      if (path === `/conf/api/v2/configs/${branch}/d/securitypolicies/`) {
        if (config && config.headers && config.headers['x-fields'] === 'id, name') {
          return Promise.resolve({data: _.map(securityPoliciesDocs, (i) => _.pick(i, 'id', 'name'))})
        }
        return Promise.resolve({data: securityPoliciesDocs})
      }
      if (path === `/conf/api/v2/configs/${branch}/d/securitypolicies/e/__default__/`) {
        return Promise.resolve({data: securityPoliciesDocs[0]})
      }
      if (path === `/conf/api/v2/configs/${branch}/d/securitypolicies/e/${docID}/v/`) {
        return Promise.resolve({data: securityPoliciesDocsLogs[0]})
      }
      if (path === `/conf/api/v2/configs/${branch}/d/flowcontrol/`) {
        if (config && config.headers && config.headers['x-fields'] === 'id, name') {
          return Promise.resolve({data: _.map(flowControlPolicyDocs, (i) => _.pick(i, 'id', 'name'))})
        }
        return Promise.resolve({data: flowControlPolicyDocs})
      }
      if (path === `/conf/api/v2/configs/${branch}/d/flowcontrol/e/c03dabe4b9ca/`) {
        return Promise.resolve({data: flowControlPolicyDocs[0]})
      }
      if (path === `/conf/api/v2/configs/${branch}/d/contentfilterprofiles/`) {
        if (config && config.headers && config.headers['x-fields'] === 'id, name') {
          return Promise.resolve({data: _.map(contentFilterDocs, (i) => _.pick(i, 'id', 'name'))})
        }
        return Promise.resolve({data: contentFilterDocs})
      }
      if (path === `/conf/api/v2/configs/${branch}/d/contentfilterprofiles/e/009e846e819e/`) {
        return Promise.resolve({data: contentFilterDocs[0]})
      }
      if (path === `/conf/api/v2/configs/${branch}/d/ratelimits/`) {
        if (config && config.headers && config.headers['x-fields'] === 'id, name') {
          return Promise.resolve({data: _.map(rateLimitsDocs, (i) => _.pick(i, 'id', 'name'))})
        }
        return Promise.resolve({data: rateLimitsDocs})
      }
      if (path === `/conf/api/v2/configs/${branch}/d/ratelimits/e/f971e92459e2/`) {
        return Promise.resolve({data: rateLimitsDocs[0]})
      }
      if (path === '/conf/api/v2/configs/master/v/') {
        return Promise.resolve({data: gitData[0].logs})
      }
      if (path === '/conf/api/v2/configs/zzz_branch/v/') {
        return Promise.resolve({data: gitData[1].logs})
      }
      return Promise.resolve({data: []})
    })
    mockRoute = {
      params: {
        branch: 'master',
        doc_type: 'aclprofiles',
        doc_id: '__default__',
      },
      path: `/config/master/aclprofiles/__default__`,
    }
    mockRouter = {
      push: jest.fn(),
    }
    wrapper = shallowMount(DocumentEditor, {
      mocks: {
        $route: mockRoute,
        $router: mockRouter,
      },
    })
    // allow all requests to finish
    setImmediate(() => {
      done()
    })
  })
  afterEach(() => {
    jest.clearAllMocks()
  })

  test('should have a git history component with correct data', () => {
    const gitHistory = wrapper.findComponent(GitHistory)
    expect(gitHistory).toBeTruthy()
    expect((gitHistory.vm as any).gitLog).toEqual(aclDocsLogs[0])
  })

  test('should send API request to restore to the correct version', async () => {
    const wantedVersion = {
      version: '7f8a987c8e5e9db7c734ac8841c543d5bc5d9657',
    }
    const putSpy = jest.spyOn(axios, 'put')
    putSpy.mockImplementation(() => Promise.resolve())
    const gitHistory = wrapper.findComponent(GitHistory)
    gitHistory.vm.$emit('restore-version', wantedVersion)
    await Vue.nextTick()
    expect(putSpy).toHaveBeenCalledWith(`/conf/api/v2/configs/master/d/aclprofiles/v/${wantedVersion.version}/revert/`)
  })

  test('should log message when receiving no configs from the server', (done) => {
    const originalLog = console.log
    let consoleOutput: string[] = []
    const mockedLog = (output: string) => consoleOutput.push(output)
    consoleOutput = []
    console.log = mockedLog
    jest.spyOn(axios, 'get').mockImplementation((path) => {
      if (path === '/conf/api/v2/configs/') {
        return Promise.reject(new Error())
      }
      return Promise.resolve({data: {}})
    })
    wrapper = shallowMount(DocumentEditor, {
      mocks: {
        $route: mockRoute,
        $router: mockRouter,
      },
    })
    // allow all requests to finish
    setImmediate(() => {
      expect(consoleOutput).toContain(`Error while attempting to get configs`)
      console.log = originalLog
      done()
    })
  })

  test('should log message when receiving no documents from the server', (done) => {
    const originalLog = console.log
    let consoleOutput: string[] = []
    const mockedLog = (output: string) => consoleOutput.push(output)
    consoleOutput = []
    console.log = mockedLog
    jest.spyOn(axios, 'get').mockImplementation((path) => {
      if (path === '/conf/api/v2/configs/') {
        return Promise.resolve({data: gitData})
      }
      const branch = (wrapper.vm as any).selectedBranch
      const doctype = (wrapper.vm as any).selectedDocType
      if (path === `/conf/api/v2/configs/${branch}/d/${doctype}/`) {
        return Promise.reject(new Error())
      }
      return Promise.resolve({data: {}})
    })
    wrapper = shallowMount(DocumentEditor, {
      mocks: {
        $route: mockRoute,
        $router: mockRouter,
      },
    })
    // allow all requests to finish
    setImmediate(() => {
      expect(consoleOutput).toContain(`Error while attempting to load documents`)
      console.log = originalLog
      done()
    })
  })

  describe('route change', () => {
    test('should not load a different path from route when it does not change', async (done) => {
      const routeChangeSpy = jest.spyOn(mockRouter, 'push')
      mockRoute.params = {
        branch: 'master',
        doc_type: 'aclprofiles',
        doc_id: '__default__',
      }
      // allow all requests to finish
      setImmediate(() => {
        expect(routeChangeSpy).not.toHaveBeenCalled()
        done()
      })
    })

    test('should load correct branch from route when changes', async (done) => {
      mockRoute.params = {
        branch: 'zzz_branch',
        doc_type: 'globalfilters',
        doc_id: '07656fbe',
      }
      // allow all requests to finish
      setImmediate(() => {
        const branchSelection = wrapper.find('.branch-selection')
        expect((branchSelection.element as HTMLSelectElement).selectedIndex).toEqual(1)
        done()
      })
    })

    test('should load correct document type from route when changes', async (done) => {
      mockRoute.params = {
        branch: 'zzz_branch',
        doc_type: 'globalfilters',
        doc_id: '07656fbe',
      }
      // allow all requests to finish
      setImmediate(() => {
        const docTypeSelection = wrapper.find('.doc-type-selection')
        expect((docTypeSelection.element as HTMLSelectElement).selectedIndex).toEqual(2)
        done()
      })
    })

    test('should load correct document id from route when changes', async (done) => {
      mockRoute.params = {
        branch: 'zzz_branch',
        doc_type: 'globalfilters',
        doc_id: '07656fbe',
      }
      // allow all requests to finish
      setImmediate(() => {
        const docSelection = wrapper.find('.doc-selection')
        expect((docSelection.element as HTMLSelectElement).selectedIndex).toEqual(1)
        done()
      })
    })

    test('should load correct branch from route without changing document type or id', async (done) => {
      mockRoute.params = {
        branch: 'zzz_branch',
        doc_type: 'aclprofiles',
        doc_id: '__default__',
      }
      // allow all requests to finish
      setImmediate(() => {
        const branchSelection = wrapper.find('.branch-selection')
        expect((branchSelection.element as HTMLSelectElement).selectedIndex).toEqual(1)
        const docTypeSelection = wrapper.find('.doc-type-selection')
        expect((docTypeSelection.element as HTMLSelectElement).selectedIndex).toEqual(0)
        const docSelection = wrapper.find('.doc-selection')
        expect((docSelection.element as HTMLSelectElement).selectedIndex).toEqual(1)
        done()
      })
    })

    test('should load correct document type from route without changing branch or document id', async (done) => {
      mockRoute.params = {
        branch: 'master',
        doc_type: 'securitypolicies',
        doc_id: '__default__',
      }
      // allow all requests to finish
      setImmediate(() => {
        const branchSelection = wrapper.find('.branch-selection')
        expect((branchSelection.element as HTMLSelectElement).selectedIndex).toEqual(0)
        const docTypeSelection = wrapper.find('.doc-type-selection')
        expect((docTypeSelection.element as HTMLSelectElement).selectedIndex).toEqual(4)
        const docSelection = wrapper.find('.doc-selection')
        expect((docSelection.element as HTMLSelectElement).selectedIndex).toEqual(0)
        done()
      })
    })

    test('should load correct document id from route without changing branch or document type', async (done) => {
      mockRoute.params = {
        branch: 'master',
        doc_type: 'aclprofiles',
        doc_id: '5828321c37e0',
      }
      // allow all requests to finish
      setImmediate(() => {
        const branchSelection = wrapper.find('.branch-selection')
        expect((branchSelection.element as HTMLSelectElement).selectedIndex).toEqual(0)
        const docTypeSelection = wrapper.find('.doc-type-selection')
        expect((docTypeSelection.element as HTMLSelectElement).selectedIndex).toEqual(0)
        const docSelection = wrapper.find('.doc-selection')
        expect((docSelection.element as HTMLSelectElement).selectedIndex).toEqual(0)
        done()
      })
    })

    test('should load correct default branch if non existent in route params', async (done) => {
      mockRoute.params = {}
      // allow all requests to finish
      setImmediate(() => {
        const branchSelection = wrapper.find('.branch-selection')
        expect((branchSelection.element as HTMLSelectElement).selectedIndex).toEqual(0)
        done()
      })
    })

    test('should load correct default document type if non existent in route params', async (done) => {
      mockRoute.params = {}
      // allow all requests to finish
      setImmediate(() => {
        const docTypeSelection = wrapper.find('.doc-type-selection')
        expect((docTypeSelection.element as HTMLSelectElement).selectedIndex).toEqual(0)
        done()
      })
    })

    test('should load correct default document id if non existent in route params', async (done) => {
      mockRoute.params = {}
      // allow all requests to finish
      setImmediate(() => {
        const docSelection = wrapper.find('.doc-selection')
        expect((docSelection.element as HTMLSelectElement).selectedIndex).toEqual(0)
        done()
      })
    })

    describe('git history when route changes', () => {
      test('should load correct git history when branch changes', (done) => {
        mockRoute.params = {
          branch: 'zzz_branch',
          doc_type: 'aclprofiles',
          doc_id: '__default__',
        }
        // allow all requests to finish
        setImmediate(() => {
          const gitHistory = wrapper.findComponent(GitHistory)
          expect(gitHistory).toBeTruthy()
          expect((gitHistory.vm as any).gitLog).toEqual(aclDocsLogs[1])
          done()
        })
      })

      test('should load correct git history when document type changes', (done) => {
        mockRoute.params = {
          branch: 'master',
          doc_type: 'securitypolicies',
          doc_id: '__default__',
        }
        // allow all requests to finish
        setImmediate(() => {
          const gitHistory = wrapper.findComponent(GitHistory)
          expect(gitHistory).toBeTruthy()
          expect((gitHistory.vm as any).gitLog).toEqual(securityPoliciesDocsLogs[0])
          done()
        })
      })

      test('should load correct git history when document id changes', (done) => {
        mockRoute.params = {
          branch: 'master',
          doc_type: 'aclprofiles',
          doc_id: '5828321c37e0',
        }
        // allow all requests to finish
        setImmediate(() => {
          const gitHistory = wrapper.findComponent(GitHistory)
          expect(gitHistory).toBeTruthy()
          expect((gitHistory.vm as any).gitLog).toEqual(aclDocsLogs[1])
          done()
        })
      })
    })
  })

  describe('branches and commits display', () => {
    test('should display correct zero amount of branches', (done) => {
      gitData = []
      jest.spyOn(axios, 'get').mockImplementation((path) => {
        if (path === '/conf/api/v2/configs/') {
          return Promise.resolve({data: gitData})
        }
        return Promise.resolve({data: []})
      })
      wrapper = shallowMount(DocumentEditor, {
        mocks: {
          $route: mockRoute,
          $router: mockRouter,
        },
      })
      // allow all requests to finish
      setImmediate(() => {
        const gitBranches = wrapper.find('.git-branches')
        expect(gitBranches.text()).toEqual('0 branches')
        done()
      })
    })

    test('should display correct zero amount of commits', (done) => {
      gitData = []
      jest.spyOn(axios, 'get').mockImplementation((path) => {
        if (path === '/conf/api/v2/configs/') {
          return Promise.resolve({data: gitData})
        }
        return Promise.resolve({data: []})
      })
      wrapper = shallowMount(DocumentEditor, {
        mocks: {
          $route: mockRoute,
          $router: mockRouter,
        },
      })
      // allow all requests to finish
      setImmediate(() => {
        const gitCommits = wrapper.find('.git-commits')
        expect(gitCommits.text()).toEqual('0 commits')
        done()
      })
    })

    test('should display correct singular amount of branches', (done) => {
      gitData = [
        {
          'id': 'master',
          'description': 'Update entry [__default__] of document [aclprofiles]',
          'date': '2020-11-10T15:49:17+02:00',
          'logs': [{
            'version': '7dd9580c00bef1049ee9a531afb13db9ef3ee956',
            'date': '2020-11-10T15:49:17+02:00',
            'parents': [
              'fc47a6cd9d7f254dd97875a04b87165cc484e075',
            ],
            'message': 'Update entry [__default__] of document [aclprofiles]',
            'email': 'curiefense@reblaze.com',
            'author': 'Curiefense API',
          }],
          'version': '7dd9580c00bef1049ee9a531afb13db9ef3ee956',
        },
      ]
      jest.spyOn(axios, 'get').mockImplementation((path) => {
        if (path === '/conf/api/v2/configs/') {
          return Promise.resolve({data: gitData})
        }
        if (path === '/conf/api/v2/configs/master/') {
          return Promise.resolve({data: gitData[0]})
        }
        if (path === '/conf/api/v2/configs/master/v/') {
          return Promise.resolve({data: gitData[0].logs})
        }
        return Promise.resolve({data: []})
      })
      wrapper = shallowMount(DocumentEditor, {
        mocks: {
          $route: mockRoute,
          $router: mockRouter,
        },
      })
      // allow all requests to finish
      setImmediate(() => {
        const gitBranches = wrapper.find('.git-branches')
        expect(gitBranches.text()).toEqual('1 branch')
        done()
      })
    })

    test('should display correct singular amount of commits', (done) => {
      gitData = [
        {
          'id': 'master',
          'description': 'Update entry [__default__] of document [aclprofiles]',
          'date': '2020-11-10T15:49:17+02:00',
          'logs': [{
            'version': '7dd9580c00bef1049ee9a531afb13db9ef3ee956',
            'date': '2020-11-10T15:49:17+02:00',
            'parents': [
              'fc47a6cd9d7f254dd97875a04b87165cc484e075',
            ],
            'message': 'Update entry [__default__] of document [aclprofiles]',
            'email': 'curiefense@reblaze.com',
            'author': 'Curiefense API',
          }],
          'version': '7dd9580c00bef1049ee9a531afb13db9ef3ee956',
        },
      ]
      jest.spyOn(axios, 'get').mockImplementation((path) => {
        if (path === '/conf/api/v2/configs/') {
          return Promise.resolve({data: gitData})
        }
        if (path === '/conf/api/v2/configs/master/') {
          return Promise.resolve({data: gitData[0]})
        }
        if (path === '/conf/api/v2/configs/master/v/') {
          return Promise.resolve({data: gitData[0].logs})
        }
        return Promise.resolve({data: []})
      })
      wrapper = shallowMount(DocumentEditor, {
        mocks: {
          $route: mockRoute,
          $router: mockRouter,
        },
      })
      // allow all requests to finish
      setImmediate(() => {
        const gitCommits = wrapper.find('.git-commits')
        expect(gitCommits.text()).toEqual('1 commit')
        done()
      })
    })

    test('should display correct plural amount of branches', () => {
      const gitBranches = wrapper.find('.git-branches')
      expect(gitBranches.text()).toEqual('2 branches')
    })

    test('should display correct plural amount of commits', () => {
      const gitCommits = wrapper.find('.git-commits')
      expect(gitCommits.text()).toEqual('10 commits')
    })
  })

  describe('dropdowns', () => {
    test('should be able to switch branches through dropdown', (done) => {
      const branchSelection = wrapper.find('.branch-selection')
      branchSelection.trigger('click')
      const options = branchSelection.findAll('option')
      options.at(1).setSelected()
      // allow all requests to finish
      setImmediate(() => {
        expect((branchSelection.element as HTMLSelectElement).selectedIndex).toEqual(1)
        done()
      })
    })

    test('should not switch doc type when switching branches', async (done) => {
      const docTypeSelection = wrapper.find('.doc-type-selection')
      docTypeSelection.trigger('click')
      const docTypeOptions = docTypeSelection.findAll('option')
      docTypeOptions.at(2).setSelected()
      await Vue.nextTick()
      const branchSelection = wrapper.find('.branch-selection')
      branchSelection.trigger('click')
      const branchOptions = branchSelection.findAll('option')
      branchOptions.at(1).setSelected()
      // allow all requests to finish
      setImmediate(() => {
        expect((docTypeSelection.element as HTMLSelectElement).selectedIndex).toEqual(2)
        done()
      })
    })

    test('should not switch selected doc when switching branches', async (done) => {
      const docSelection = wrapper.find('.doc-selection')
      docSelection.trigger('click')
      const docOptions = docSelection.findAll('option')
      docOptions.at(1).setSelected()
      await Vue.nextTick()
      const branchSelection = wrapper.find('.branch-selection')
      branchSelection.trigger('click')
      const branchOptions = branchSelection.findAll('option')
      branchOptions.at(1).setSelected()
      // allow all requests to finish
      setImmediate(() => {
        expect((docSelection.element as HTMLSelectElement).selectedIndex).toEqual(1)
        done()
      })
    })

    test('should be able to switch doc types through dropdown', (done) => {
      const docTypeSelection = wrapper.find('.doc-type-selection')
      docTypeSelection.trigger('click')
      const options = docTypeSelection.findAll('option')
      options.at(2).setSelected()
      // allow all requests to finish
      setImmediate(() => {
        expect((docTypeSelection.element as HTMLSelectElement).selectedIndex).toEqual(2)
        done()
      })
    })

    test('should be able to switch docs through dropdown', (done) => {
      // switch to profiling lists
      const docTypeSelection = wrapper.find('.doc-type-selection')
      docTypeSelection.trigger('click')
      const docTypeOptions = docTypeSelection.findAll('option')
      docTypeOptions.at(2).setSelected()
      // allow all requests to finish
      setImmediate(() => {
        // switch to a different document
        const docSelection = wrapper.find('.doc-selection')
        docSelection.trigger('click')
        const options = docSelection.findAll('option')
        options.at(1).setSelected()
        // allow all requests to finish
        setImmediate(() => {
          expect((docSelection.element as HTMLSelectElement).selectedIndex).toEqual(1)
          done()
        })
      })
    })
  })

  describe('buttons', () => {
    test('should refresh referenced IDs lists after saving security policy entity', (done) => {
      // switch to security policy entity type
      const docTypeSelection = wrapper.find('.doc-type-selection')
      docTypeSelection.trigger('click')
      const options = docTypeSelection.findAll('option')
      options.at(4).setSelected()
      // allow all requests to finish
      setImmediate(async () => {
        const doc = (wrapper.vm as any).selectedDoc
        doc.name = `${doc.name} changed`
        jest.spyOn(axios, 'put').mockImplementation(() => Promise.resolve())
        const getSpy = jest.spyOn(axios, 'get')
        const saveDocumentButton = wrapper.find('.save-document-button')
        saveDocumentButton.trigger('click')
        await Vue.nextTick()
        expect(getSpy).toHaveBeenCalledWith(`/conf/api/v2/configs/master/d/securitypolicies/`)
        done()
      })
    })

    test('should be able to save document changes', async () => {
      const doc = (wrapper.vm as any).selectedDoc
      doc.name = `${doc.name} changed`
      const putSpy = jest.spyOn(axios, 'put')
      putSpy.mockImplementation(() => Promise.resolve())
      const saveDocumentButton = wrapper.find('.save-document-button')
      saveDocumentButton.trigger('click')
      await Vue.nextTick()
      expect(putSpy).toHaveBeenCalledWith(`/conf/api/v2/configs/master/d/aclprofiles/e/${doc.id}/`, doc)
    })

    test('should be able to fork document', async () => {
      const originalDoc = (wrapper.vm as any).selectedDoc
      const forkedDoc = {...originalDoc}
      forkedDoc.id = expect.any(String)
      forkedDoc.name = `copy of ${forkedDoc.name}`
      const postSpy = jest.spyOn(axios, 'post')
      postSpy.mockImplementation(() => Promise.resolve())
      const forkDocumentButton = wrapper.find('.fork-document-button')
      forkDocumentButton.trigger('click')
      await Vue.nextTick()
      expect(postSpy).toHaveBeenCalledWith(`/conf/api/v2/configs/master/d/aclprofiles/e/`, forkedDoc)
    })

    test('should change security policy match when forking security policy document', (done) => {
      // switching to security policies
      const docTypeSelection = wrapper.find('.doc-type-selection')
      docTypeSelection.trigger('click')
      const options = docTypeSelection.findAll('option')
      options.at(4).setSelected()
      // allow all requests to finish
      setImmediate(async () => {
        const originalDoc = (wrapper.vm as any).selectedDoc
        const forkedDoc = {...originalDoc}
        forkedDoc.id = expect.any(String)
        forkedDoc.name = `copy of ${forkedDoc.name}`
        forkedDoc.match = expect.stringContaining(`.${forkedDoc.match}`)
        const postSpy = jest.spyOn(axios, 'post')
        postSpy.mockImplementation(() => Promise.resolve())
        const forkDocumentButton = wrapper.find('.fork-document-button')
        forkDocumentButton.trigger('click')
        await Vue.nextTick()
        expect(postSpy).toHaveBeenCalledWith(`/conf/api/v2/configs/master/d/securitypolicies/e/`, forkedDoc)
        done()
      })
    })

    test('should be able to add a new document', async () => {
      const newDoc = DatasetsUtils.newDocEntryFactory.aclprofiles()
      newDoc.id = expect.any(String)
      const postSpy = jest.spyOn(axios, 'post')
      postSpy.mockImplementation(() => Promise.resolve())
      const newDocumentButton = wrapper.find('.new-document-button')
      newDocumentButton.trigger('click')
      await Vue.nextTick()
      expect(postSpy).toHaveBeenCalledWith(`/conf/api/v2/configs/master/d/aclprofiles/e/`, newDoc)
    })

    test('should be able to add multiple new documents in a row with different IDs', async () => {
      const newDocIDs: string[] = []
      const postSpy = jest.spyOn(axios, 'post')
      postSpy.mockImplementation((url, data: Partial<Document>) => {
        newDocIDs.push(data.id)
        return Promise.resolve()
      })
      const newDocumentButton = wrapper.find('.new-document-button')
      newDocumentButton.trigger('click')
      await Vue.nextTick()
      newDocumentButton.trigger('click')
      await Vue.nextTick()
      expect(postSpy).toHaveBeenCalledTimes(2)
      expect(newDocIDs[0]).not.toEqual(newDocIDs[1])
    })

    test('should be able to delete a document', async () => {
      const deleteSpy = jest.spyOn(axios, 'delete')
      deleteSpy.mockImplementation(() => Promise.resolve())
      // create new document so we can delete it
      const newDocumentButton = wrapper.find('.new-document-button')
      newDocumentButton.trigger('click')
      await Vue.nextTick()
      const docID = (wrapper.vm as any).selectedDocID
      const deleteDocumentButton = wrapper.find('.delete-document-button')
      deleteDocumentButton.trigger('click')
      await Vue.nextTick()
      expect(deleteSpy).toHaveBeenCalledWith(`/conf/api/v2/configs/master/d/aclprofiles/e/${docID}/`)
    })

    test('should not be able to delete a document if its id is __default__', async () => {
      const deleteSpy = jest.spyOn(axios, 'delete')
      deleteSpy.mockImplementation(() => Promise.resolve())
      wrapper.setData({selectedDocID: '__default__'})
      await Vue.nextTick()
      const deleteDocumentButton = wrapper.find('.delete-document-button')
      deleteDocumentButton.trigger('click')
      await Vue.nextTick()
      expect(deleteSpy).not.toHaveBeenCalled()
    })

    test('should not be able to delete an ACL Profile document if it is referenced by a security policy', async () => {
      // switch to a different document
      const docSelection = wrapper.find('.doc-selection')
      docSelection.trigger('click')
      const options = docSelection.findAll('option')
      options.at(1).setSelected()
      await Vue.nextTick()
      const deleteSpy = jest.spyOn(axios, 'delete')
      deleteSpy.mockImplementation(() => Promise.resolve())
      wrapper.setData({selectedDoc: {id: '__default__'}})
      const deleteDocumentButton = wrapper.find('.delete-document-button')
      deleteDocumentButton.trigger('click')
      await Vue.nextTick()
      expect(deleteSpy).not.toHaveBeenCalled()
    })

    test('should not be able to delete an Content Filter Profile document if it is referenced by a security policy', async () => {
      // switch to Content Filter Profiles
      const docTypeSelection = wrapper.find('.doc-type-selection')
      docTypeSelection.trigger('click')
      const docTypeOptions = docTypeSelection.findAll('option')
      docTypeOptions.at(5).setSelected()
      await Vue.nextTick()
      await Vue.nextTick()
      const deleteSpy = jest.spyOn(axios, 'delete')
      deleteSpy.mockImplementation(() => Promise.resolve())
      wrapper.setData({selectedDoc: {id: '009e846e819e'}})
      const deleteDocumentButton = wrapper.find('.delete-document-button')
      deleteDocumentButton.trigger('click')
      await Vue.nextTick()
      expect(deleteSpy).not.toHaveBeenCalled()
    })

    test('should not be able to delete a Rate Limit document if it is referenced by a security policy', async () => {
      // switch to Rate Limits
      const docTypeSelection = wrapper.find('.doc-type-selection')
      docTypeSelection.trigger('click')
      const docTypeOptions = docTypeSelection.findAll('option')
      docTypeOptions.at(3).setSelected()
      await Vue.nextTick()
      await Vue.nextTick()
      const deleteSpy = jest.spyOn(axios, 'delete')
      deleteSpy.mockImplementation(() => Promise.resolve())
      wrapper.setData({selectedDoc: {id: 'f971e92459e2'}})
      const deleteDocumentButton = wrapper.find('.delete-document-button')
      deleteDocumentButton.trigger('click')
      await Vue.nextTick()
      expect(deleteSpy).not.toHaveBeenCalled()
    })

    test('should not attempt to download document when download button is clicked' +
      ' if the full docs data was not loaded yet', async () => {
      jest.spyOn(axios, 'get').mockImplementation((path, config) => {
        if (path === '/conf/api/v2/configs/') {
          return Promise.resolve({data: gitData})
        }
        const branch = (wrapper.vm as any).selectedBranch
        const docID = (wrapper.vm as any).selectedDocID
        if (path === `/conf/api/v2/configs/${branch}/d/aclprofiles/`) {
          if (config && config.headers && config.headers['x-fields'] === 'id, name') {
            return Promise.resolve({data: _.map(aclDocs, (i) => _.pick(i, 'id', 'name'))})
          }
          setTimeout(() => {
            return Promise.resolve({data: aclDocs})
          }, 5000)
        }
        if (path === `/conf/api/v2/configs/${branch}/d/aclprofiles/e/__default__/`) {
          return Promise.resolve({data: aclDocs[0]})
        }
        if (path === `/conf/api/v2/configs/${branch}/d/aclprofiles/e/5828321c37e0/`) {
          return Promise.resolve({data: aclDocs[1]})
        }
        if (path === `/conf/api/v2/configs/${branch}/d/aclprofiles/v/7f8a987c8e5e9db7c734ac8841c543d5bc5d9657/`) {
          return Promise.resolve({data: aclGitOldVersion})
        }
        if (path === `/conf/api/v2/configs/${branch}/d/aclprofiles/e/${docID}/v/`) {
          return Promise.resolve({data: aclDocsLogs[0]})
        }
        return Promise.resolve({data: []})
      })
      wrapper = shallowMount(DocumentEditor, {
        mocks: {
          $route: mockRoute,
          $router: mockRouter,
        },
      })
      await Vue.nextTick()
      const downloadFileSpy = jest.spyOn(Utils, 'downloadFile').mockImplementation(() => {
      })
      // force update because downloadFile is mocked after it is read to to be used as event handler
      await (wrapper.vm as any).$forceUpdate()
      await Vue.nextTick()
      const downloadDocButton = wrapper.find('.download-doc-button')
      downloadDocButton.trigger('click')
      await Vue.nextTick()
      expect(downloadFileSpy).not.toHaveBeenCalled()
    })

    test('should attempt to download document when download button is clicked', async () => {
      const wantedFileName = 'aclprofiles'
      const wantedFileType = 'json'
      const wantedFileData = aclDocs
      const downloadFileSpy = jest.spyOn(Utils, 'downloadFile').mockImplementation(() => {
      })
      // force update because downloadFile is mocked after it is read to to be used as event handler
      await (wrapper.vm as any).$forceUpdate()
      await Vue.nextTick()
      const downloadDocButton = wrapper.find('.download-doc-button')
      downloadDocButton.trigger('click')
      await Vue.nextTick()
      expect(downloadFileSpy).toHaveBeenCalledWith(wantedFileName, wantedFileType, wantedFileData)
    })
  })

  describe('no data', () => {
    test('should display correct message when there is no branch data', (done) => {
      jest.spyOn(axios, 'get').mockImplementation((path) => {
        if (path === '/conf/api/v2/configs/') {
          return Promise.resolve({data: []})
        }
        return Promise.resolve({data: []})
      })
      wrapper = shallowMount(DocumentEditor, {
        mocks: {
          $route: mockRoute,
          $router: mockRouter,
        },
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

    test('should display link to version control when there is no branch data', (done) => {
      jest.spyOn(axios, 'get').mockImplementation((path) => {
        if (path === '/conf/api/v2/configs/') {
          return Promise.resolve({data: []})
        }
        return Promise.resolve({data: []})
      })
      wrapper = shallowMount(DocumentEditor, {
        mocks: {
          $route: mockRoute,
          $router: mockRouter,
        },
      })
      // allow all requests to finish
      setImmediate(async () => {
        jest.spyOn(mockRouter, 'push').mockImplementation((path) => {
          expect(path).toEqual('/versioncontrol')
          done()
        })
        const button = wrapper.find('.version-control-referral-button')
        button.trigger('click')
        await Vue.nextTick()
      })
    })

    test('should display correct message when there is no doc type data', (done) => {
      // it is not possible to get to this state from the UI, but we protect from it anyway
      (wrapper.vm as any).selectedDocType = null
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
      jest.spyOn(axios, 'get').mockImplementation((path) => {
        if (path === '/conf/api/v2/configs/') {
          return Promise.resolve({data: gitData})
        }
        return Promise.resolve({data: []})
      })
      wrapper = shallowMount(DocumentEditor, {
        mocks: {
          $route: mockRoute,
          $router: mockRouter,
        },
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
      jest.spyOn(axios, 'get').mockImplementation((path) => {
        if (path === '/conf/api/v2/configs/') {
          return new Promise(() => {
          })
        }
        return Promise.resolve({data: []})
      })
      wrapper = shallowMount(DocumentEditor, {
        mocks: {
          $route: mockRoute,
          $router: mockRouter,
        },
      })
      await Vue.nextTick()
      const docLoadingIndicator = wrapper.find('.document-loading')
      expect(docLoadingIndicator.element).toBeDefined()
    })

    test('should display loading indicator when doc not loaded', async () => {
      jest.spyOn(axios, 'get').mockImplementation((path) => {
        if (path === '/conf/api/v2/configs/') {
          return Promise.resolve({data: gitData})
        }
        const branch = (wrapper.vm as any).selectedBranch
        if (path === `/conf/api/v2/configs/${branch}/d/aclprofiles/`) {
          return new Promise(() => {
          })
        }
        return Promise.resolve({data: []})
      })
      wrapper = shallowMount(DocumentEditor, {
        mocks: {
          $route: mockRoute,
          $router: mockRouter,
        },
      })
      await Vue.nextTick()
      const docLoadingIndicator = wrapper.find('.document-loading')
      expect(docLoadingIndicator.element).toBeDefined()
    })

    test('should display loading indicator when saving document changes', async () => {
      jest.spyOn(axios, 'put').mockImplementation(() => new Promise(() => {
      }))
      const saveDocumentButton = wrapper.find('.save-document-button')
      saveDocumentButton.trigger('click')
      await Vue.nextTick()
      expect(saveDocumentButton.element.classList).toContain('is-loading')
    })

    test('should display loading indicator when forking document', async () => {
      jest.spyOn(axios, 'post').mockImplementation(() => new Promise(() => {
      }))
      const forkDocumentButton = wrapper.find('.fork-document-button')
      forkDocumentButton.trigger('click')
      await Vue.nextTick()
      expect(forkDocumentButton.element.classList).toContain('is-loading')
    })

    test('should display loading indicator when adding a new document', async () => {
      jest.spyOn(axios, 'post').mockImplementation(() => new Promise(() => {
      }))
      const newDocumentButton = wrapper.find('.new-document-button')
      newDocumentButton.trigger('click')
      await Vue.nextTick()
      expect(newDocumentButton.element.classList).toContain('is-loading')
    })

    test('should display loading indicator when deleting a document', async () => {
      jest.spyOn(axios, 'delete').mockImplementation(() => new Promise(() => {
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
