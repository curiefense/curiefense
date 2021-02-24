import {ToastType} from 'bulma-toast'
import * as bulmaToast from 'bulma-toast'

const invalidityClasses = ` has-text-danger has-background-danger-light`

// Validates an input based on given validator (Function / Boolean) and adds necessary classes if input is invalid
const validateInput = (event: Event, validator: Function | boolean) => {
  let className = (event.target as HTMLElement)?.className
  let isValid
  className = className.replace(`${invalidityClasses}`, '')
  if (typeof validator === 'function') {
    isValid = validator(event)
  } else {
    isValid = validator
  }
  if (!isValid) {
    className += `${invalidityClasses}`
  }
  (event.target as HTMLElement).className = className
  return isValid
}

// Clear invalidity classes from an input
const clearInputValidationClasses = (element: HTMLElement) => {
  let className = element.className
  className = className.replace(`${invalidityClasses}`, '')
  element.className = className
}

// Generates a unique name in a given entities list
const generateUniqueEntityName = (originalName: string, entitiesList: string[],
                                  isCopy?: boolean, divider = ' ') => {
  if (!originalName) {
    originalName = `new${divider}entity`
  }
  let namePrefix = ''
  if (isCopy) {
    namePrefix = `copy${divider}of${divider}`
  }
  let newName = `${namePrefix}${originalName}`
  let counter = 1
  while (entitiesList.includes(newName)) {
    counter++
    newName = `${namePrefix}${originalName}(${counter})`
  }
  return newName
}

// Download data as file
const downloadFile = (fileName: string, fileType: string, data: any) => {
  // Check if file type can be downloaded
  const recognizedDownloadFileTypes = ['json']
  if (!recognizedDownloadFileTypes.includes(fileType)) {
    console.log('Unable to download file, unknown file type')
    return
  }
  // Create downloadable content based on file type
  let content: BlobPart = ''
  if (fileType === 'json') {
    content = JSON.stringify(data)
  }
  // Create anchor element with download data
  const blob = new Blob([content], {
    type: `application/${fileType}`,
  })
  const link = document.createElement('a')
  link.href = window.URL.createObjectURL(blob)
  link.download = `${fileName}.${fileType}`
  // Click initiates the download
  link.click()
}

// Displays a toast message at the bottom left corner of the page
const toast = (message: string, type: ToastType) => {
  bulmaToast.toast({
    message: message,
    type: <ToastType>`is-light ${type}`,
    position: 'bottom-left',
    closeOnClick: true,
    pauseOnHover: true,
    duration: 3000,
    opacity: 0.8,
  })
}

// Displays the default toast with a success color
const successToast = (message: string) => {
  toast(message, 'is-success')
}

// Displays the default toast with a failure color
const failureToast = (message: string) => {
  toast(message, 'is-danger')
}

export default {
  name: 'Utils',
  validateInput,
  clearInputValidationClasses,
  generateUniqueEntityName,
  downloadFile,
  toast,
  successToast,
  failureToast,
}
