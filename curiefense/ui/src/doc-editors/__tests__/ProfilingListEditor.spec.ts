import ProfilingListEditor from '@/doc-editors/ProfilingListEditor.vue'
import {beforeEach, describe, expect, jest, test} from '@jest/globals'
import {shallowMount, Wrapper} from '@vue/test-utils'
import {TagRule, TagRuleSectionEntry} from '@/types'
import Vue from 'vue'
import ResponseAction from '@/components/ResponseAction.vue'
import TagsAutocompleteInput from '@/components/TagAutocompleteInput.vue'
import TagAutocompleteInput from '@/components/TagAutocompleteInput.vue'
import EntriesRelationList from '@/components/EntriesRelationList.vue'
import axios from 'axios'

jest.mock('axios')

describe('ProfilingListEditor.vue', () => {
  let docs: TagRule[]
  let wrapper: Wrapper<Vue>
  beforeEach(() => {
    docs = [{
      'id': 'xlbp148c',
      'name': 'API Discovery',
      'source': 'self-managed',
      'mdate': '2020-05-23T00:04:41',
      'notes': 'Tag API Requests',
      'active': true,
      'tags': ['api', 'okay'],
      'action': {
        'type': 'monitor',
        'params': {},
      },
      'rule': {
        'relation': 'OR',
        'sections': [
          {'relation': 'OR', 'entries': [['ip', '1.1.1.1', null]]},
          {'relation': 'OR', 'entries': [['ip', '2.2.2.2', null]]},
          {'relation': 'OR', 'entries': [['headers', ['headerrr', 'valueeee'], 'anooo']]},
        ],
      },
    }, {
      'id': '07656fbe',
      'name': 'devop internal demo',
      'source': 'self-managed',
      'mdate': '2020-05-23T00:04:41',
      'notes': 'this is my own list',
      'active': false,
      'tags': ['internal', 'devops'],
      'action': {
        'type': 'monitor',
        'params': {},
      },
      'rule': {
        'relation': 'OR',
        'sections': [
          {
            'relation': 'OR',
            'entries': [
              ['ip', '1.1.1.1', null],
            ],
          },
          {
            'relation': 'OR',
            'entries': [
              ['ip', '2.2.2.2', null],
            ],
          },
          {
            'relation': 'OR',
            'entries': [
              ['headers', ['headerrr', 'valueeee'], 'anooo'],
            ],
          }],
      },
    }]
    wrapper = shallowMount(ProfilingListEditor, {
      propsData: {
        selectedDoc: docs[0],
      },
    })
  })

  describe('form data', () => {
    test('should have correct ID displayed', () => {
      expect(wrapper.find('.document-id').text()).toEqual(docs[0].id)
    })

    test('should have correct name in input', () => {
      const element = wrapper.find('.document-name').element as HTMLInputElement
      expect(element.value).toEqual(docs[0].name)
    })

    test('should have correct active mode', () => {
      const element = wrapper.find('.document-active').element as HTMLInputElement
      expect(element.checked).toEqual(docs[0].active)
    })

    test('should have correct sections relation mode selected', () => {
      const container = wrapper.find('.document-sections-relation')
      // AND - span at 0
      // OR - span at 1
      const element = container.findAll('span').at(1).element as HTMLElement
      expect(element.classList).toContain('is-selected')
    })

    test('should have tags input component with correct data', () => {
      const tagsAutocompleteInputComponent = wrapper.findComponent(TagsAutocompleteInput)
      expect(tagsAutocompleteInputComponent.props('initialTag')).toEqual(docs[0].tags.join(' '))
    })

    test('should have correct source in input', () => {
      const element = wrapper.find('.document-source').element as HTMLInputElement
      expect(element.value).toEqual(docs[0].source)
    })

    test('should have response action component with correct data', () => {
      const responseActionComponent = wrapper.findComponent(ResponseAction)
      expect(responseActionComponent.props('action')).toEqual(docs[0].action)
    })

    test('should have correct notes in input', () => {
      const element = wrapper.find('.document-notes').element as HTMLTextAreaElement
      expect(element.value).toEqual(docs[0].notes.toString())
    })

    test('should have entries relation component with correct data', () => {
      const entriesRelationListComponent = wrapper.findComponent(EntriesRelationList)
      expect(entriesRelationListComponent.props('rule')).toEqual(docs[0].rule)
    })

    describe('sections entries display', () => {
      test('should display correct zero amount of sections', async () => {
        docs[0].rule.sections = []
        wrapper = shallowMount(ProfilingListEditor, {
          propsData: {
            selectedDoc: docs[0],
          },
        })
        await Vue.nextTick()
        const display = wrapper.find('.sections-entries-display')
        expect(display.text()).toContain('0 sections')
      })

      test('should display correct zero amount of entries', async () => {
        docs[0].rule.sections = [
          {'relation': 'OR', 'entries': []},
        ]
        wrapper = shallowMount(ProfilingListEditor, {
          propsData: {
            selectedDoc: docs[0],
          },
        })
        await Vue.nextTick()
        const display = wrapper.find('.sections-entries-display')
        expect(display.text()).toContain('0 entries')
      })

      test('should display correct singular amount of sections', async () => {
        docs[0].rule.sections = [
          {'relation': 'OR', 'entries': [['ip', '1.1.1.1', null]]},
        ]
        wrapper = shallowMount(ProfilingListEditor, {
          propsData: {
            selectedDoc: docs[0],
          },
        })
        await Vue.nextTick()
        const display = wrapper.find('.sections-entries-display')
        expect(display.text()).toContain('1 section')
      })

      test('should display correct singular amount of entries', async () => {
        docs[0].rule.sections = [
          {'relation': 'OR', 'entries': [['ip', '1.1.1.1', null]]},
        ]
        wrapper = shallowMount(ProfilingListEditor, {
          propsData: {
            selectedDoc: docs[0],
          },
        })
        await Vue.nextTick()
        const display = wrapper.find('.sections-entries-display')
        expect(display.text()).toContain('1 entry')
      })

      test('should display correct plural amount of sections', () => {
        const display = wrapper.find('.sections-entries-display')
        expect(display.text()).toContain('3 sections')
      })

      test('should display correct plural amount of entries', () => {
        const display = wrapper.find('.sections-entries-display')
        expect(display.text()).toContain('3 entries')
      })
    })
  })

  describe('form data when non editable', () => {
    // non editable if source is not 'self-managed'
    beforeEach(async () => {
      docs[0].source = 'https://example.com'
      wrapper = shallowMount(ProfilingListEditor, {
        propsData: {
          selectedDoc: docs[0],
        },
      })
      await Vue.nextTick()
    })

    test('should hide sections relation mode', () => {
      const container = wrapper.find('.document-sections-relation')
      expect(container.exists()).toBeFalsy()
    })

    test('should hide remove all sections button', async () => {
      const button = wrapper.find('.remove-all-sections-button')
      expect(button.exists()).toBeFalsy()
    })

    test('should display update now button', async () => {
      const button = wrapper.find('.update-now-button')
      expect(button.exists()).toBeTruthy()
    })

    test('should have entries relation component with editable false prop', () => {
      const entriesRelationListComponent = wrapper.findComponent(EntriesRelationList)
      expect(entriesRelationListComponent.props('editable')).toBeFalsy()
    })
  })

  describe('tags management', () => {
    test('should emit doc update when adding tags', async () => {
      const newTag = 'test-tag'
      const newTagInputValue = `${docs[0].tags.join(' ')} ${newTag}`
      const wantedEmit = JSON.parse(JSON.stringify(docs[0]))
      wantedEmit.tags.push(newTag)
      // change tags
      const tagAutocompleteInput = wrapper.findComponent(TagAutocompleteInput)
      tagAutocompleteInput.vm.$emit('tag-changed', newTagInputValue)
      await Vue.nextTick()
      // check
      expect(wrapper.emitted('update:selectedDoc')).toBeTruthy()
      expect(wrapper.emitted('update:selectedDoc')[0]).toEqual([wantedEmit])
    })

    test('should emit form validness when EntriesRelationList is validated', async () => {
      const entriesRelationListComponent = wrapper.findComponent(EntriesRelationList)
      entriesRelationListComponent.vm.$emit('invalid', false)
      await Vue.nextTick()
      // check
      expect(wrapper.emitted('form-invalid')).toBeTruthy()
    })

    test('should set document tags to be an empty array if empty string provided', async () => {
      const newTagInputValue = ''
      const wantedEmit = JSON.parse(JSON.stringify(docs[0]))
      wantedEmit.tags = []
      // change tags
      const tagAutocompleteInput = wrapper.findComponent(TagAutocompleteInput)
      tagAutocompleteInput.vm.$emit('tag-changed', newTagInputValue)
      await Vue.nextTick()
      // check
      expect(wrapper.emitted('update:selectedDoc')).toBeTruthy()
      expect(wrapper.emitted('update:selectedDoc')[0]).toEqual([wantedEmit])
    })

    test('should set tags input to be an empty string if document tags do not exist', async () => {
      delete docs[0].tags
      wrapper = shallowMount(ProfilingListEditor, {
        propsData: {
          selectedDoc: docs[0],
        },
      })
      await Vue.nextTick()
      const tagAutocompleteInput = wrapper.findComponent(TagAutocompleteInput)
      expect(tagAutocompleteInput.props('initialTag')).toEqual('')
    })

    test('should set tags input to be an empty string if document tags is empty', async () => {
      docs[0].tags = []
      wrapper = shallowMount(ProfilingListEditor, {
        propsData: {
          selectedDoc: docs[0],
        },
      })
      await Vue.nextTick()
      const tagAutocompleteInput = wrapper.findComponent(TagAutocompleteInput)
      expect(tagAutocompleteInput.props('initialTag')).toEqual('')
    })
  })

  describe('rule relation', () => {
    // AND - span at 0
    // OR - span at 1
    let container: Wrapper<Vue>
    let andElement: Wrapper<Vue>
    let orElement: Wrapper<Vue>
    beforeEach(() => {
      container = wrapper.find('.document-sections-relation')
      andElement = container.findAll('span').at(0)
      orElement = container.findAll('span').at(1)
    })

    test('should correctly switch between rule relations status using space bar keypress', async () => {
      expect(andElement.element.classList).not.toContain('is-selected')
      expect(orElement.element.classList).toContain('is-selected')
      container.trigger('keypress.space')
      await wrapper.vm.$forceUpdate()
      expect(andElement.element.classList).toContain('is-selected')
      expect(orElement.element.classList).not.toContain('is-selected')
      container.trigger('keypress.space')
      await wrapper.vm.$forceUpdate()
      expect(andElement.element.classList).not.toContain('is-selected')
      expect(orElement.element.classList).toContain('is-selected')
    })

    test('should correctly switch between rule relations status using enter keypress', async () => {
      expect(andElement.element.classList).not.toContain('is-selected')
      expect(orElement.element.classList).toContain('is-selected')
      container.trigger('keypress.enter')
      await wrapper.vm.$forceUpdate()
      expect(andElement.element.classList).toContain('is-selected')
      expect(orElement.element.classList).not.toContain('is-selected')
      container.trigger('keypress.enter')
      await wrapper.vm.$forceUpdate()
      expect(andElement.element.classList).not.toContain('is-selected')
      expect(orElement.element.classList).toContain('is-selected')
    })

    test('should correctly switch to AND state when span is clicked', async () => {
      expect(andElement.element.classList).not.toContain('is-selected')
      expect(orElement.element.classList).toContain('is-selected')
      andElement.trigger('click')
      await wrapper.vm.$forceUpdate()
      expect(andElement.element.classList).toContain('is-selected')
      expect(orElement.element.classList).not.toContain('is-selected')
      andElement.trigger('click')
      await wrapper.vm.$forceUpdate()
      expect(andElement.element.classList).toContain('is-selected')
      expect(orElement.element.classList).not.toContain('is-selected')
    })

    test('should correctly switch to OR state when span is clicked', async () => {
      andElement.trigger('click')
      await wrapper.vm.$forceUpdate()
      expect(andElement.element.classList).toContain('is-selected')
      expect(orElement.element.classList).not.toContain('is-selected')
      orElement.trigger('click')
      await wrapper.vm.$forceUpdate()
      expect(andElement.element.classList).not.toContain('is-selected')
      expect(orElement.element.classList).toContain('is-selected')
      orElement.trigger('click')
      await wrapper.vm.$forceUpdate()
      expect(andElement.element.classList).not.toContain('is-selected')
      expect(orElement.element.classList).toContain('is-selected')
    })
  })

  test('should remove all entries relation data from component when clear button is clicked', async () => {
    const wantedRule: TagRule['rule'] = {
      relation: docs[0].rule.relation,
      sections: [],
    }
    const button = wrapper.find('.remove-all-sections-button')
    button.trigger('click')
    await Vue.nextTick()
    const entriesRelationListComponent = wrapper.findComponent(EntriesRelationList)
    expect(entriesRelationListComponent.props('rule')).toEqual(wantedRule)
  })

  describe('update now button', () => {
    let resolveData: any
    beforeEach(async () => {
      resolveData = {}
      jest.spyOn(axios, 'get').mockImplementation((path) => {
        if (path.includes('/conf/api/v1/tools/fetch')) {
          return Promise.resolve(resolveData)
        }
        return Promise.resolve({data: {}})
      })
      docs[0].source = 'https://example.com'
      wrapper = shallowMount(ProfilingListEditor, {
        propsData: {
          selectedDoc: docs[0],
        },
      })
      await Vue.nextTick()
    })

    test('should update entries relation component with correct data - ipv4 array', async () => {
      const wantedEntries: TagRuleSectionEntry[] = [
        [
          'ip',
          '203.208.60.0/24',
          'Crawler',
        ],
        [
          'ip',
          '209.85.238.0/24',
          'Crawler',
        ],
        [
          'ip',
          '66.249.90.0',
          'Crawler',
        ],
      ]
      const wantedData: TagRule['rule'] = {
        relation: 'OR',
        sections: [{
          entries: wantedEntries,
          relation: 'OR',
        }],
      }
      resolveData = {
        data: {
          'type': 'acl',
          'name': 'Crawler example',
          'id': 'example_id',
          'active': true,
          'mdate': '2020-11-04 07:54:27.417791',
          'source': 'https://example.com',
          'notes': 'some example crawlers',
          'entries_relation': 'OR',
          'tags': [
            'allowlist',
            'crawler',
          ],
          'entries': wantedEntries,
        },
      }
      const button = wrapper.find('.update-now-button')
      button.trigger('click')
      await Vue.nextTick()
      await wrapper.vm.$forceUpdate()
      await Vue.nextTick()
      const entriesRelationListComponent = wrapper.findComponent(EntriesRelationList)
      expect(entriesRelationListComponent.props('rule')).toEqual(wantedData)
    })

    test('should update entries relation component with correct data - ipv6 array', async () => {
      const wantedEntries: TagRuleSectionEntry[] = [
        [
          'ip',
          '2603:8080:3d40:f7:d45b:477e:579e:245',
          'Crawler',
        ],
        [
          'ip',
          '2a01:4f8:150:8147::2',
          'Crawler',
        ],
        [
          'ip',
          '2001:16b8:a08a:cb00:a5ce:3228:fbdc:23ad',
          'Crawler',
        ],
      ]
      const wantedData: TagRule['rule'] = {
        relation: 'OR',
        sections: [{
          entries: wantedEntries,
          relation: 'OR',
        }],
      }
      resolveData = {
        data: {
          'type': 'acl',
          'name': 'Crawler example',
          'id': 'example_id',
          'active': true,
          'mdate': '2020-11-04 07:54:27.417791',
          'source': 'https://example.com',
          'notes': 'some example crawlers',
          'entries_relation': 'OR',
          'tags': [
            'allowlist',
            'crawler',
          ],
          'entries': wantedEntries,
        },
      }
      const button = wrapper.find('.update-now-button')
      button.trigger('click')
      await Vue.nextTick()
      await wrapper.vm.$forceUpdate()
      await Vue.nextTick()
      const entriesRelationListComponent = wrapper.findComponent(EntriesRelationList)
      expect(entriesRelationListComponent.props('rule')).toEqual(wantedData)
    })

    test('should update entries relation component with correct data - asn array', async () => {
      const wantedEntries: TagRuleSectionEntry[] = [
        [
          'asn',
          'as34109',
          'spam',
        ],
        [
          'asn',
          'as3396',
          'spam',
        ],
        [
          'asn',
          'as3502',
          'spam',
        ],
      ]
      const wantedData: TagRule['rule'] = {
        relation: 'OR',
        sections: [{
          entries: wantedEntries,
          relation: 'OR',
        }],
      }
      resolveData = {
        data: {
          'type': 'acl',
          'name': 'ASN example',
          'id': 'example_id',
          'active': true,
          'mdate': '2020-11-04 07:54:27.417791',
          'source': 'https://example.com',
          'notes': 'some example ASNs',
          'entries_relation': 'OR',
          'tags': [
            'blocklist',
            'ASN',
          ],
          'entries': wantedEntries,
        },
      }
      const button = wrapper.find('.update-now-button')
      button.trigger('click')
      await Vue.nextTick()
      await wrapper.vm.$forceUpdate()
      await Vue.nextTick()
      const entriesRelationListComponent = wrapper.findComponent(EntriesRelationList)
      expect(entriesRelationListComponent.props('rule')).toEqual(wantedData)
    })

    test('should update entries relation component with correct data - ip singles', async () => {
      const wantedEntries: TagRuleSectionEntry[] = [
        [
          'ip',
          '203.208.60.0/24',
          null,
        ],
        [
          'ip',
          '209.85.238.0/24',
          null,
        ],
        [
          'ip',
          '66.249.90.0/24',
          null,
        ],
      ]
      const wantedData: TagRule['rule'] = {
        relation: 'OR',
        sections: [{
          entries: wantedEntries,
          relation: 'OR',
        }],
      }
      resolveData = {
        data: {
          'foo': 'bar',
          'data1': wantedEntries[0][1],
          'data2': wantedEntries[1][1],
          'data3': wantedEntries[2][1],
        },
      }
      const button = wrapper.find('.update-now-button')
      button.trigger('click')
      await Vue.nextTick()
      await wrapper.vm.$forceUpdate()
      await Vue.nextTick()
      const entriesRelationListComponent = wrapper.findComponent(EntriesRelationList)
      expect(entriesRelationListComponent.props('rule')).toEqual(wantedData)
    })

    test('should update entries relation component with correct data - asn singles', async () => {
      const wantedEntries: TagRuleSectionEntry[] = [
        [
          'asn',
          'as34109',
          null,
        ],
        [
          'asn',
          'as3396',
          null,
        ],
        [
          'asn',
          'as3502',
          null,
        ],
      ]
      const wantedData: TagRule['rule'] = {
        relation: 'OR',
        sections: [{
          entries: wantedEntries,
          relation: 'OR',
        }],
      }
      resolveData = {
        data: {
          'foo': 'bar',
          'data1': wantedEntries[0][1],
          'data2': wantedEntries[1][1],
          'data3': wantedEntries[2][1],
        },
      }
      const button = wrapper.find('.update-now-button')
      button.trigger('click')
      await Vue.nextTick()
      await wrapper.vm.$forceUpdate()
      await Vue.nextTick()
      const entriesRelationListComponent = wrapper.findComponent(EntriesRelationList)
      expect(entriesRelationListComponent.props('rule')).toEqual(wantedData)
    })

    test('should update entries relation component with correct data - ip inside object', async () => {
      const wantedEntries: TagRuleSectionEntry[] = [
        [
          'ip',
          '203.208.60.0/24',
          'Crawler',
        ],
        [
          'ip',
          '209.85.238.0/24',
          'Crawler',
        ],
        [
          'ip',
          '66.249.90.0/24',
          'Crawler',
        ],
      ]
      const wantedData: TagRule['rule'] = {
        relation: 'OR',
        sections: [{
          entries: wantedEntries,
          relation: 'OR',
        }],
      }
      resolveData = {
        data: {
          'type': 'acl',
          'name': 'Crawler example',
          'id': 'example_id',
          'active': true,
          'mdate': '2020-11-04 07:54:27.417791',
          'source': 'https://example.com',
          'notes': 'some example crawlers',
          'entries_relation': 'OR',
          'tags': [
            'allowlist',
            'crawler',
          ],
          'entries': {
            '1': [wantedEntries[0], wantedEntries[1]],
            '2': wantedEntries[2],
          },
        },
      }
      const button = wrapper.find('.update-now-button')
      button.trigger('click')
      await Vue.nextTick()
      await wrapper.vm.$forceUpdate()
      await Vue.nextTick()
      const entriesRelationListComponent = wrapper.findComponent(EntriesRelationList)
      expect(entriesRelationListComponent.props('rule')).toEqual(wantedData)
    })

    test('should update entries relation component with correct data - asn inside object', async () => {
      const wantedEntries: TagRuleSectionEntry[] = [
        [
          'asn',
          'as34109',
          'spam',
        ],
        [
          'asn',
          'as3396',
          'spam',
        ],
        [
          'asn',
          'as3502',
          'spam',
        ],
      ]
      const wantedData: TagRule['rule'] = {
        relation: 'OR',
        sections: [{
          entries: wantedEntries,
          relation: 'OR',
        }],
      }
      resolveData = {
        data: {
          'type': 'acl',
          'name': 'ASN example',
          'id': 'example_id',
          'active': true,
          'mdate': '2020-11-04 07:54:27.417791',
          'source': 'https://example.com',
          'notes': 'some example ASNs',
          'entries_relation': 'OR',
          'tags': [
            'blocklist',
            'ASN',
          ],
          'entries': {
            '1': [wantedEntries[0], wantedEntries[1]],
            '2': wantedEntries[2],
          },
        },
      }
      const button = wrapper.find('.update-now-button')
      button.trigger('click')
      await Vue.nextTick()
      await wrapper.vm.$forceUpdate()
      await Vue.nextTick()
      const entriesRelationListComponent = wrapper.findComponent(EntriesRelationList)
      expect(entriesRelationListComponent.props('rule')).toEqual(wantedData)
    })

    test('should update entries relation component with correct data - ip array', async () => {
      const wantedEntries: TagRuleSectionEntry[] = [
        [
          'ip',
          '203.208.60.0/24',
          'Crawler',
        ],
        [
          'ip',
          '209.85.238.0/24',
          'Crawler',
        ],
        [
          'ip',
          '66.249.90.0/24',
          'Crawler',
        ],
        [
          'ip',
          '66.249.91.0/24',
          null,
        ],
      ]
      const wantedData: TagRule['rule'] = {
        relation: 'OR',
        sections: [{
          entries: wantedEntries,
          relation: 'OR',
        }],
      }
      resolveData = {
        data: `${wantedEntries[0][1]}#${wantedEntries[0][2]}\n` +
          `${wantedEntries[1][1]};${wantedEntries[1][2]}\n` +
          `${wantedEntries[2][1]}?${wantedEntries[2][2]}\n` +
          `${wantedEntries[3][1]}`,
      }
      const button = wrapper.find('.update-now-button')
      button.trigger('click')
      await Vue.nextTick()
      await wrapper.vm.$forceUpdate()
      await Vue.nextTick()
      const entriesRelationListComponent = wrapper.findComponent(EntriesRelationList)
      expect(entriesRelationListComponent.props('rule')).toEqual(wantedData)
    })

    test('should update entries relation component with correct data - asn array', async () => {
      const wantedEntries: TagRuleSectionEntry[] = [
        [
          'asn',
          'as34109',
          'spam',
        ],
        [
          'asn',
          'as3396',
          'spam',
        ],
        [
          'asn',
          'as3502',
          'spam',
        ],
        [
          'asn',
          'as6752',
          null,
        ],
      ]
      const wantedData: TagRule['rule'] = {
        relation: 'OR',
        sections: [{
          entries: wantedEntries,
          relation: 'OR',
        }],
      }
      resolveData = {
        data: `${wantedEntries[0][1]}#${wantedEntries[0][2]}\n` +
          `${wantedEntries[1][1]};${wantedEntries[1][2]}\n` +
          `${wantedEntries[2][1]}?${wantedEntries[2][2]}\n` +
          `${wantedEntries[3][1]}`,
      }
      const button = wrapper.find('.update-now-button')
      button.trigger('click')
      await Vue.nextTick()
      await wrapper.vm.$forceUpdate()
      await Vue.nextTick()
      const entriesRelationListComponent = wrapper.findComponent(EntriesRelationList)
      expect(entriesRelationListComponent.props('rule')).toEqual(wantedData)
    })

    test('should not update entries relation component when no data found', async () => {
      const wantedData: TagRule['rule'] = docs[0].rule
      resolveData = {
        data: {
          'type': 'acl',
          'name': 'Crawler example',
          'id': 'example_id',
          'active': true,
          'mdate': '2020-11-04 07:54:27.417791',
          'source': 'https://example.com',
          'notes': 'some example crawlers',
          'entries_relation': 'OR',
          'tags': [
            'allowlist',
            'crawler',
          ],
          'entries': [],
        },
      }
      const button = wrapper.find('.update-now-button')
      button.trigger('click')
      await Vue.nextTick()
      await wrapper.vm.$forceUpdate()
      await Vue.nextTick()
      const entriesRelationListComponent = wrapper.findComponent(EntriesRelationList)
      expect(entriesRelationListComponent.props('rule')).toEqual(wantedData)
    })
  })
})
