import axios from 'axios'
import DatasetsUtils from '@/assets/DatasetsUtils'
import * as bulmaToast from 'bulma-toast'

const apiRoot = DatasetsUtils.ConfAPIRoot
const apiVersion = DatasetsUtils.ConfAPIVersion
const logsApiRoot = DatasetsUtils.LogsAPIRoot
const logsApiVersion = DatasetsUtils.LogsAPIVersion
const axiosMethodsMap = {
    'GET': axios.get,
    'PUT': axios.put,
    'POST': axios.post,
    'DELETE': axios.delete,
}

function toast(message, type) {
    bulmaToast.toast(
        {
            message: message,
            type: `is-light ${type}`,
            position: 'bottom-left',
            closeOnClick: true,
            pauseOnHover: true,
            duration: 3000,
            opacity: 0.8,
        }
    )
}

function successToast(message) {
    toast(message, 'is-success')
}

function failureToast(message) {
    toast(message, 'is-danger')
}

function processRequest(methodName, apiUrl, data, config, successMessage, failureMessage) {

    // Get correct axios method
    if (!methodName) {
        methodName = 'GET'
    } else {
        methodName = methodName.toUpperCase()
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
        request = axiosMethod(apiUrl)
    }
    request = request
        .then((response) => {
            // Toast message
            if (successMessage) {
                successToast(successMessage)
            }
            return response
        })
        .catch((error) => {
            // Toast message
            if (failureMessage) {
                failureToast(failureMessage)
            }
            throw error
        })
    return request
}

function sendRequest(methodName, urlTail, data, config, successMessage, failureMessage) {
    const apiUrl = `${apiRoot}/${apiVersion}/${urlTail}`
    return processRequest(methodName, apiUrl, data, config, successMessage, failureMessage)
}

function sendLogsRequest(methodName, urlTail, data, config, successMessage, failureMessage) {
    const apiUrl = `${logsApiRoot}/${logsApiVersion}/${urlTail}`
    return processRequest(methodName, apiUrl, data, config, successMessage, failureMessage)
}

export default {
    name: 'RequestsUtils',
    sendRequest,
    sendLogsRequest,
}
