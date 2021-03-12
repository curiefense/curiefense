import axios, {AxiosRequestConfig, AxiosResponse} from 'axios'
import Utils from '@/assets/Utils'

export type MethodNames = 'GET' | 'PUT' | 'POST' | 'DELETE'

const confAPIRoot = '/conf/api'
const confAPIVersion = 'v1'
const logsAPIRoot = '/logs/api'
const logsAPIVersion = 'v1'

const axiosMethodsMap: Record<MethodNames, Function> = {
  'GET': axios.get,
  'PUT': axios.put,
  'POST': axios.post,
  'DELETE': axios.delete,
}

const processRequest = (methodName: MethodNames, apiUrl: string, data: any, config: AxiosRequestConfig,
                        successMessage: string, failureMessage: string) => {
  // Get correct axios method
  if (!methodName) {
    methodName = 'GET'
  } else {
    methodName = <MethodNames>methodName.toUpperCase()
  }
  const axiosMethod = axiosMethodsMap[methodName]
  if (!axiosMethod) {
    console.error(`Attempted sending unrecognized request method ${methodName}`)
    return
  }

  // Request
  console.log(`Sending ${methodName} request to url ${apiUrl}`)
  let request
  if (data) {
    if (config) {
      request = axiosMethod(apiUrl, data, config)
    } else {
      request = axiosMethod(apiUrl, data)
    }
  } else {
    if (config) {
      request = axiosMethod(apiUrl, config)
    } else {
      request = axiosMethod(apiUrl)
    }
  }
  request = request.then((response: AxiosResponse) => {
    // Toast message
    if (successMessage) {
      Utils.successToast(successMessage)
    }
    return response
  }).catch((error: Error) => {
    // Toast message
    if (failureMessage) {
      Utils.failureToast(failureMessage)
    }
    throw error
  })
  return request
}

const sendRequest = (methodName: MethodNames, urlTail: string, data?: any, config?: AxiosRequestConfig,
                     successMessage?: string, failureMessage?: string) => {
  const apiUrl = `${confAPIRoot}/${confAPIVersion}/${urlTail}`
  return processRequest(methodName, apiUrl, data, config, successMessage, failureMessage)
}

export default {
  name: 'RequestsUtils',
  sendRequest,
  confAPIRoot,
  confAPIVersion,
  logsAPIRoot,
  logsAPIVersion,
}
