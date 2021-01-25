import ProfilingListEditor from '@/doc-editors/ProfilingListEditor'
import {describe, expect, test} from '@jest/globals'
import {mount} from '@vue/test-utils'

describe('ProfilingListEditor.vue', () => {
    const docs = [{
        'id': 'xlbp148c',
        'name': 'API Discovery',
        'source': 'self-managed',
        'mdate': '2020-05-23T00:04:41',
        'notes': 'Tag API Requests',
        'active': true,
        'tags': ['api'],
        'rule': {
            'relation': 'OR',
            'sections': [
                {'relation': 'OR', 'entries': [['ip', '1.1.1.1', null]]},
                {'relation': 'OR', 'entries': [['ip', '2.2.2.2', null]]},
                {'relation': 'OR', 'entries': [['headers', ['headerrr', 'valueeee'], 'anooo']]}]
        }
    }, {
        'id': '07656fbe',
        'name': 'devop internal demo',
        'source': 'self-managed',
        'mdate': '2020-05-23T00:04:41',
        'notes': 'this is my own list',
        'active': false,
        'tags': ['internal', 'devops'],
        'rule': {
            'relation': 'OR',
            'sections': [
                {'relation': 'OR', 'entries': [['ip', '1.1.1.1', null]]},
                {'relation': 'OR', 'entries': [['ip', '2.2.2.2', null]]},
                {'relation': 'OR', 'entries': [['headers', ['headerrr', 'valueeee'], 'anooo']]}]
        }
    }]

    test('should exist - STUB', () => {
        const wrapper = mount(ProfilingListEditor, {
            propsData: {
                selectedDoc: docs[0]
            }
        })
        expect(wrapper).toBeTruthy()
    })
})
