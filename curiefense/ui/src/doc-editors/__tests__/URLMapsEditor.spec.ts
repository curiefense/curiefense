import URLMapsEditor from '@/doc-editors/URLMapsEditor'
import {describe, test, expect} from '@jest/globals'
import {mount} from '@vue/test-utils'

describe('URLMapsEditor.vue', () => {
    const docs = [{
        'id': '__default__',
        'name': 'default entry',
        'match': '__default__',
        'map': [{
            'name': 'default',
            'match': '/',
            'acl_profile': '__default__',
            'acl_active': false,
            'waf_profile': '__default__',
            'waf_active': false,
            'limit_ids': []
        }]
    }]

    test('should exist - STUB', () => {
        const wrapper = mount(URLMapsEditor, {
            propsData: {
                selectedDoc: docs[0]
            }
        })
        expect(wrapper).toBeTruthy()
    })
})
