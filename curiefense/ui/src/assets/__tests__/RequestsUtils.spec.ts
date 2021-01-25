import RequestsUtils from '@/assets/RequestsUtils'
import {afterEach, beforeEach, describe, expect, jest, test} from '@jest/globals'
import DatasetsUtils from '@/assets/DatasetsUtils'
import * as bulmaToast from 'bulma-toast'

jest.mock('axios')
import axios from 'axios'

describe('RequestsUtils.ts', () => {
    let getSpy
    let putSpy
    let postSpy
    let deleteSpy
    beforeEach(() => {
        getSpy = jest.spyOn(axios, 'get')
        axios.get.mockImplementation(() => Promise.resolve())
        putSpy = jest.spyOn(axios, 'put')
        axios.put.mockImplementation(() => Promise.resolve())
        postSpy = jest.spyOn(axios, 'post')
        axios.post.mockImplementation(() => Promise.resolve())
        deleteSpy = jest.spyOn(axios, 'delete')
        axios.delete.mockImplementation(() => Promise.resolve())
    })
    afterEach(() => {
        jest.clearAllMocks()
    })

    function buildFuncDescribe(requestFunc, baseUrl, urlTrail) {
        describe(`${requestFunc.name} function`, () => {
            const apiUrl = `${baseUrl}${urlTrail}`
            const dataObject = {
                id: 'a240gava',
                name: 'A name'
            }

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

                test('should send POST request correctly with header config', async () => {
                    const config = {headers: {'x-fields': 'name'}}
                    await requestFunc('POST', urlTrail, dataObject, config)
                    expect(postSpy).toHaveBeenCalledWith(apiUrl, dataObject, config)
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
                let consoleOutput = []
                const mockedLog = output => consoleOutput.push(output)
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
                    console.error = output => consoleOutput.push(output)

                    const weirdRequestMethod = 'POTATO'
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
                const originalToast = bulmaToast.toast
                let toastOutput = []
                const mockedToast = output => toastOutput.push(output)
                beforeEach(() => {
                    toastOutput = []
                    bulmaToast.toast = mockedToast
                })
                afterEach(() => {
                    bulmaToast.toast = originalToast
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

                test('should send failure toast when GET request is rejected', (done) => {
                    axios.get.mockImplementationOnce(() => Promise.reject())
                    requestFunc('GET', urlTrail, null, null, successMessage, failureMessage)
                        .catch(() => {
                            expect(toastOutput[0].message).toContain(failureMessage)
                            expect(toastOutput[0].type).toContain(failureMessageClass)
                            done()
                        })
                })

                test('should send failure toast when PUT request is rejected', (done) => {
                    axios.put.mockImplementationOnce(() => Promise.reject())
                    requestFunc('PUT', urlTrail, dataObject, null, successMessage, failureMessage)
                        .catch(() => {
                            expect(toastOutput[0].message).toContain(failureMessage)
                            expect(toastOutput[0].type).toContain(failureMessageClass)
                            done()
                        })
                })

                test('should send failure toast when POST request is rejected', (done) => {
                    axios.post.mockImplementationOnce(() => Promise.reject())
                    requestFunc('POST', urlTrail, dataObject, null, successMessage, failureMessage)
                        .catch(() => {
                            expect(toastOutput[0].message).toContain(failureMessage)
                            expect(toastOutput[0].type).toContain(failureMessageClass)
                            done()
                        })
                })

                test('should send failure toast when DELETE request is rejected', (done) => {
                    axios.delete.mockImplementationOnce(() => Promise.reject())
                    requestFunc('DELETE', urlTrail, null, null, successMessage, failureMessage)
                        .catch(() => {
                            expect(toastOutput[0].message).toContain(failureMessage)
                            expect(toastOutput[0].type).toContain(failureMessageClass)
                            done()
                        })
                })
            })
        })
    }

    const sendRequestBaseUrl = `${DatasetsUtils.ConfAPIRoot}/${DatasetsUtils.ConfAPIVersion}/`
    const sendRequestUrlTrail = 'configs/master/'
    buildFuncDescribe(RequestsUtils.sendRequest, sendRequestBaseUrl, sendRequestUrlTrail)

    const sendLogsRequestBaseUrl = `${DatasetsUtils.LogsAPIRoot}/${DatasetsUtils.LogsAPIVersion}/`
    const sendLogsRequestUrlTrail = 'exec/'
    buildFuncDescribe(RequestsUtils.sendLogsRequest, sendLogsRequestBaseUrl, sendLogsRequestUrlTrail)
})
