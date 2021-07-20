import GitHistory from '@/components/GitHistory.vue'
import {describe, test, expect, beforeEach} from '@jest/globals'
import {mount, Wrapper} from '@vue/test-utils'

describe('GitHistory.vue', () => {
  // Number of log items = 7
  const gitLog = [
    {
      'version': '7dd9580c00bef1049ee9a531afb13db9ef3ee956',
      'date': '2020-11-10T15:49:17+02:00',
      'parents': [
        'fc47a6cd9d7f254dd97875a04b87165cc484e075',
      ],
      'message': 'Update entry [__default__] of document [aclpolicies]',
      'email': 'curiefense@reblaze.com',
      'author': 'Curiefense API 1',
    },
    {
      'version': 'fc47a6cd9d7f254dd97875a04b87165cc484e075',
      'date': '2020-11-10T15:48:35+02:00',
      'parents': [
        '5aba4a5b9d6faea1896ee8965c7aa651f76af63c',
      ],
      'message': 'Update entry [__default__] of document [aclpolicies]',
      'email': 'curiefense@reblaze.com',
      'author': 'Curiefense API 2',
    },
    {
      'version': '5aba4a5b9d6faea1896ee8965c7aa651f76af63c',
      'date': '2020-11-10T15:48:31+02:00',
      'parents': [
        '277c5d7bd0e2eb4b9d2944f7eefdfadf37ba8581',
      ],
      'message': 'Update entry [__default__] of document [aclpolicies]',
      'email': 'curiefense@reblaze.com',
      'author': 'Curiefense API 3',
    },
    {
      'version': '277c5d7bd0e2eb4b9d2944f7eefdfadf37ba8581',
      'date': '2020-11-10T15:48:22+02:00',
      'parents': [
        '878b47deeddac94625fe7c759786f2df885ec541',
      ],
      'message': 'Update entry [__default__] of document [aclpolicies]',
      'email': 'curiefense@reblaze.com',
      'author': 'Curiefense API 4',
    },
    {
      'version': '878b47deeddac94625fe7c759786f2df885ec541',
      'date': '2020-11-10T15:48:05+02:00',
      'parents': [
        '93c180513fe7edeaf1c0ca69a67aa2a11374da4f',
      ],
      'message': 'Update entry [__default__] of document [aclpolicies]',
      'email': 'curiefense@reblaze.com',
      'author': 'Curiefense API 5',
    },
    {
      'version': '93c180513fe7edeaf1c0ca69a67aa2a11374da4f',
      'date': '2020-11-10T15:47:59+02:00',
      'parents': [
        '1662043d2a18d6ad2c9c94d6f826593ff5506354',
      ],
      'message': 'Update entry [__default__] of document [aclpolicies]',
      'email': 'curiefense@reblaze.com',
      'author': 'Curiefense API 6',
    },
    {
      'version': '1662043d2a18d6ad2c9c94d6f826593ff5506354',
      'date': '2020-11-08T21:31:41+01:00',
      'parents': [
        '16379cdf39501574b4a2f5a227b82a4454884b84',
      ],
      'message': 'Create config [master]\n',
      'email': 'curiefense@reblaze.com',
      'author': 'Curiefense API 7',
    },
  ]
  const apiPath = '/conf/api/v1/configs/master/d/aclpolicies/e/__default__/v/'
  let wrapper: Wrapper<Vue>
  beforeEach(() => {
    wrapper = mount(GitHistory, {
      propsData: {
        gitLog,
        apiPath,
      },
    })
  })

  describe('log table rendering', () => {
    test('should only render five rows in addition to header ' +
      'if the log has more than 5 rows of data', () => {
      expect(wrapper.findAll('tr').length).toEqual(6)
    })

    test('should render all rows if table expanded in addition to header', async () => {
      const loadMore = wrapper.find('.load-more a')
      await loadMore.trigger('click')
      expect(wrapper.findAll('tr').length).toEqual(8)
    })

    test('should render footer with expand message ' +
      'if table is not expanded and more than five items are present', () => {
      const loadMore = wrapper.find('.load-more a')
      expect(loadMore.text()).toEqual('Load More')
    })

    test('should not render footer with expand message if table is already expanded', async () => {
      await wrapper.find('.load-more a').trigger('click')
      expect(wrapper.find('.load-more').exists()).toBeFalsy()
    })

    test('should open commit details', async () => {
      expect(wrapper.find('.commit-details').exists()).toBeFalsy()
      await wrapper.find('tr:first-child .view-details').trigger('click')
      expect(wrapper.find('.commit-details').exists()).toBeTruthy()
    })

    test('should open accordion on click on its header', async () => {
      const collapsingButton = wrapper.find('.collapsible-header a')
      expect( collapsingButton.text() ).toContain( 'Show Version History' )
      await collapsingButton.trigger( 'click' )
      expect( collapsingButton.text() ).toContain( 'Hide Version History' )
    })

    test('should not render footer if less than five items are present', async () => {
      const shortGitLog = gitLog.slice(0, 4)
      wrapper = mount(GitHistory, {
        propsData: {
          gitLog: shortGitLog,
          apiPath,
        },
      })
      expect(wrapper.find('.load-more').exists()).toBeFalsy()
    })
  })

  describe('log version restoration', () => {
    test('should emit a restore-version event when restore button is clicked', async () => {
      // 0 is the table header, 1 is our first data
      const firstDataRow = wrapper.findAll('tr').at(1)
      const restoreButton = firstDataRow.find('.restore-button')
      await restoreButton.trigger('click')
      expect(wrapper.emitted('restore-version')).toBeTruthy()
      expect(wrapper.emitted('restore-version')[0]).toEqual([gitLog[0]])
    })
  })

  describe('git history sorting', () => {
    const AUTHOR_CELL = 4
    test('should sort table data by author in desc order', async () => {
      const authorColumnHeader = wrapper.findAll( 'th' ).at( AUTHOR_CELL )
      await authorColumnHeader.trigger( 'click' )
      expect(wrapper.findAll('tr:first-child td').at( AUTHOR_CELL ).text()).toEqual( gitLog[gitLog.length-1].author )
      expect(wrapper.findAll( 'th' ).at( AUTHOR_CELL ).classes()).toContain( 'desc' )
    })
    test('should change order to asc after second click', async () => {
      const authorColumnHeader = wrapper.findAll( 'th' ).at( AUTHOR_CELL )
      await authorColumnHeader.trigger( 'click' )
      await authorColumnHeader.trigger( 'click' )
      expect(wrapper.findAll( 'tr:first-child td' ).at( AUTHOR_CELL ).text()).toEqual( gitLog[0].author )
      expect(wrapper.findAll( 'th' ).at( AUTHOR_CELL ).classes()).toContain( 'asc' )
    })
    test('should not be sortable if there is the only history entry', async () => {
      const shortGitLog = gitLog.slice(0, 1)
      wrapper = mount(GitHistory, {
        propsData: {
          gitLog: shortGitLog,
          apiPath,
        },
      })
      const authorColumnHeader = wrapper.findAll( 'th' ).at( AUTHOR_CELL )
      await authorColumnHeader.trigger( 'click' )
      expect(wrapper.findAll( '.asc, .desc' ).exists()).toBeFalsy()
    })
  })
})
