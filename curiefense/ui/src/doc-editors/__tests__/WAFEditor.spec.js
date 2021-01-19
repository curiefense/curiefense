import WAFEditor from '@/doc-editors/WAFEditor'
import {beforeEach, describe, expect, test} from '@jest/globals'
import {shallowMount} from '@vue/test-utils'

describe('WAFEditor.vue', () => {
    let docs
    let wrapper
    beforeEach(() => {
        docs = [{
            'id': '__default__',
            'name': 'default waf',
            'ignore_alphanum': true,
            'max_header_length': 1024,
            'max_cookie_length': 2048,
            'max_arg_length': 1536,
            'max_headers_count': 36,
            'max_cookies_count': 42,
            'max_args_count': 512,
            'args': {'names': [], 'regex': []},
            'headers': {'names': [], 'regex': []},
            'cookies': {'names': [], 'regex': []}
        }]
        wrapper = shallowMount(WAFEditor, {
            propsData: {
                selectedDoc: docs[0]
            }
        })
    })

    describe('form data', () => {
        test('should have correct ID displayed', () => {
            expect(wrapper.find('.document-id').text()).toEqual(docs[0].id)
        })

        test('should have correct name in input', () => {
            expect(wrapper.find('.document-name').element.value).toEqual(docs[0].name)
        })

        test('should have correct max header length in input', () => {
            expect(wrapper.find('.max-header-length-input').element.value).toEqual(docs[0].max_header_length.toString())
        })

        test('should have correct max cookie length in input', () => {
            expect(wrapper.find('.max-cookie-length-input').element.value).toEqual(docs[0].max_cookie_length.toString())
        })

        test('should have correct max arg length in input', () => {
            expect(wrapper.find('.max-arg-length-input').element.value).toEqual(docs[0].max_arg_length.toString())
        })

        test('should have correct max headers count in input', () => {
            expect(wrapper.find('.max-headers-count-input').element.value).toEqual(docs[0].max_headers_count.toString())
        })

        test('should have correct max cookies count in input', () => {
            expect(wrapper.find('.max-cookies-count-input').element.value).toEqual(docs[0].max_cookies_count.toString())
        })

        test('should have correct max args count in input', () => {
            expect(wrapper.find('.max-args-count-input').element.value).toEqual(docs[0].max_args_count.toString())
        })
    })
})
