import Publish from '@/views/Publish.vue'
import {describe, test, expect, jest} from '@jest/globals'
import {mount} from '@vue/test-utils'

jest.mock('axios')
import axios from 'axios'

describe('Publish.vue', () => {
  test('should exist - STUB', () => {
    const data = {
      data: {
        'buckets': [
          {
            'name': 'prod',
            'url': 's3://curiefense-test01/prod',
          },
          {
            'name': 'devops',
            'url': 's3://curiefense-test01/devops',
          },
        ],
        'branch_buckets': [
          {
            'name': 'master',
            'buckets': [
              'prod',
            ],
          },
          {
            'name': 'devops',
            'buckets': [
              'devops',
            ],
          },
        ],
      },
    }
    jest.spyOn(axios, 'get').mockImplementation(() => Promise.resolve(data))
    const wrapper = mount(Publish)
    expect(wrapper).toBeTruthy()
  })
})
