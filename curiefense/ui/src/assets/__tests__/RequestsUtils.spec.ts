import RequestsUtils, {IRequestParams} from '../../assets/RequestsUtils'
import {afterEach, beforeEach, describe, expect, jest, test} from '@jest/globals'
import * as bulmaToast from 'bulma-toast'
import {Options} from 'bulma-toast'
import axios from 'axios'

jest.mock('axios')

describe('RequestsUtils.ts', () => {
  let getSpy: any
  let putSpy: any
  let postSpy: any
  let deleteSpy: any
  beforeEach(() => {
    getSpy = jest.spyOn(axios, 'get').mockImplementation(() => Promise.resolve())
    putSpy = jest.spyOn(axios, 'put').mockImplementation(() => Promise.resolve())
    postSpy = jest.spyOn(axios, 'post').mockImplementation(() => Promise.resolve())
    deleteSpy = jest.spyOn(axios, 'delete').mockImplementation(() => Promise.resolve())
  })
  afterEach(() => {
    jest.clearAllMocks()
  })

  function buildFuncDescribe(func: Function, baseUrl: string, urlTrail: string) {
    describe(`${func.name} function`, () => {
      const apiUrl = `${baseUrl}${urlTrail}`
      const dataObject = {
        id: 'a240gava',
        name: 'A name',
      }
      const requestFunc = (
        methodName: string,
        url: IRequestParams['url'],
        data?: IRequestParams['data'],
        config?: IRequestParams['config'],
        successMessage?: IRequestParams['successMessage'],
        failureMessage?: IRequestParams['failureMessage'],
        undoFunction?: IRequestParams['undoFunction'],
        onFail?: IRequestParams['onFail'],
      ) => func({methodName, url, data, config, successMessage, failureMessage, undoFunction, onFail})

      describe('basic usage', () => {
        test('should send GET request correctly', async () => {
          await requestFunc('GET', urlTrail)
          expect(getSpy).toHaveBeenCalledWith(apiUrl)
        })

        test('should send PUT request correctly', async () => {
          await requestFunc('PUT', urlTrail, dataObject)
          expect(putSpy).toHaveBeenCalledWith(apiUrl, dataObject)
        })

        test('should send POST request correctly', async () => {
          await requestFunc('POST', urlTrail, dataObject)
          expect(postSpy).toHaveBeenCalledWith(apiUrl, dataObject)
        })

        test('should send DELETE request correctly', async () => {
          await requestFunc('DELETE', urlTrail)
          expect(deleteSpy).toHaveBeenCalledWith(apiUrl)
        })

        test('should send POST request correctly with header config and data', async () => {
          const config = {headers: {'x-fields': 'name'}}
          await requestFunc('POST', urlTrail, dataObject, config)
          expect(postSpy).toHaveBeenCalledWith(apiUrl, dataObject, config)
        })

        test('should send POST request correctly with header config without data', async () => {
          const config = {headers: {'x-fields': 'name'}}
          await requestFunc('POST', urlTrail, null, config)
          expect(postSpy).toHaveBeenCalledWith(apiUrl, config)
        })

        test('should send GET request correctly when method name is not capitalized', async () => {
          await requestFunc('get', urlTrail)
          expect(getSpy).toHaveBeenCalledWith(apiUrl)
        })

        test('should send GET request correctly when method name is not prompted', async () => {
          await requestFunc(null, urlTrail)
          expect(getSpy).toHaveBeenCalledWith(apiUrl)
        })
      })

      describe('logging', () => {
        const originalLog = console.log
        let consoleOutput: string[] = []
        const mockedLog = (output: string) => consoleOutput.push(output)
        beforeEach(() => {
          consoleOutput = []
          console.log = mockedLog
        })
        afterEach(() => {
          console.log = originalLog
        })

        test('should log GET request correctly', async () => {
          await requestFunc('GET', urlTrail)
          expect(consoleOutput).toContain(`Sending GET request to url ${apiUrl}`)
        })

        test('should log PUT request correctly', async () => {
          await requestFunc('PUT', urlTrail, dataObject)
          expect(consoleOutput).toContain(`Sending PUT request to url ${apiUrl}`)
        })

        test('should log POST request correctly', async () => {
          await requestFunc('POST', urlTrail, dataObject)
          expect(consoleOutput).toContain(`Sending POST request to url ${apiUrl}`)
        })

        test('should log DELETE request correctly', async () => {
          await requestFunc('DELETE', urlTrail)
          expect(consoleOutput).toContain(`Sending DELETE request to url ${apiUrl}`)
        })

        test('should send error when attempting to send request with unrecognized request method', async () => {
          const originalError = console.error
          console.error = (output: string) => consoleOutput.push(output)
          const weirdRequestMethod = 'POTATO' as IRequestParams['methodName']
          await requestFunc(weirdRequestMethod, urlTrail)
          expect(consoleOutput).toContain(`Attempted sending unrecognized request method ${weirdRequestMethod}`)
          console.error = originalError
        })
      })

      describe('toast messages', () => {
        const successMessage = 'yay we did it!'
        const successMessageClass = 'is-success'
        const failureMessage = 'oops, something went wrong'
        const failureMessageClass = 'is-danger'
        let toastOutput: Options[] = []
        beforeEach(() => {
          toastOutput = []
          jest.spyOn(bulmaToast, 'toast').mockImplementation((output: Options) => {
            toastOutput.push(output)
          })
        })
        afterEach(() => {
          jest.clearAllMocks()
        })

        test('should not send success toast when GET request is rejected if not set', async () => {
          await requestFunc('GET', urlTrail, null, null, null, failureMessage)
          expect(toastOutput.length).toEqual(0)
        })

        test('should send success toast when GET request returns successfully', async () => {
          await requestFunc('GET', urlTrail, null, null, successMessage, failureMessage)
          expect(toastOutput[0].message).toContain(successMessage)
          expect(toastOutput[0].type).toContain(successMessageClass)
        })

        test('should send success toast when PUT request returns successfully', async () => {
          await requestFunc('PUT', urlTrail, dataObject, null, successMessage, failureMessage)
          expect(toastOutput[0].message).toContain(successMessage)
          expect(toastOutput[0].type).toContain(successMessageClass)
        })

        test('should send success toast when POST request returns successfully', async () => {
          await requestFunc('POST', urlTrail, dataObject, null, successMessage, failureMessage)
          expect(toastOutput[0].message).toContain(successMessage)
          expect(toastOutput[0].type).toContain(successMessageClass)
        })

        test('should send success toast when DELETE request returns successfully', async () => {
          await requestFunc('DELETE', urlTrail, null, null, successMessage, failureMessage)
          expect(toastOutput[0].message).toContain(successMessage)
          expect(toastOutput[0].type).toContain(successMessageClass)
        })

        test('should not send failure toast when GET request is rejected if not set', (done) => {
          jest.spyOn(axios, 'get').mockImplementationOnce(() => Promise.reject(new Error()))
          const onFail = () => {
            expect(toastOutput.length).toEqual(0)
            done()
          }
          requestFunc('GET', urlTrail, null, null, successMessage, null, null, onFail)
        })

        test('should send failure toast when GET request is rejected', (done) => {
          jest.spyOn(axios, 'get').mockImplementationOnce(() => Promise.reject(new Error()))
          const onFail = () => {
            expect(toastOutput[0].message).toContain(failureMessage)
            expect(toastOutput[0].type).toContain(failureMessageClass)
            done()
          }
          requestFunc('GET', urlTrail, null, null, successMessage, failureMessage, null, onFail)
        })

        test('should send failure toast when PUT request is rejected', (done) => {
          jest.spyOn(axios, 'put').mockImplementationOnce(() => Promise.reject(new Error()))
          const onFail = () => {
            expect(toastOutput[0].message).toContain(failureMessage)
            expect(toastOutput[0].type).toContain(failureMessageClass)
            done()
          }
          requestFunc('PUT', urlTrail, dataObject, null, successMessage, failureMessage, null, onFail)
        })

        test('should send failure toast when POST request is rejected', (done) => {
          jest.spyOn(axios, 'post').mockImplementationOnce(() => Promise.reject(new Error()))
          const onFail = () => {
            expect(toastOutput[0].message).toContain(failureMessage)
            expect(toastOutput[0].type).toContain(failureMessageClass)
            done()
          }
          requestFunc('POST', urlTrail, dataObject, null, successMessage, failureMessage, null, onFail)
        })

        test('should send failure toast when DELETE request is rejected', (done) => {
          jest.spyOn(axios, 'delete').mockImplementationOnce(() => Promise.reject(new Error()))
          const onFail = () => {
            expect(toastOutput[0].message).toContain(failureMessage)
            expect(toastOutput[0].type).toContain(failureMessageClass)
            done()
          }
          requestFunc('DELETE', urlTrail, null, null, successMessage, failureMessage, null, onFail)
        })
      })
    })
  }

  const sendRequestBaseUrl = `${RequestsUtils.confAPIRoot}/${RequestsUtils.confAPIVersion}/`
  const sendRequestUrlTrail = 'configs/master/'
  buildFuncDescribe(RequestsUtils.sendRequest, sendRequestBaseUrl, sendRequestUrlTrail)
})
