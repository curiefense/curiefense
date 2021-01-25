// Validates an input based on given validator (Function / Boolean) and adds necessary classes if input is invalid
const validateInput = (event, validator) => {
    let className = event.target.className
    let isValid
    className = className.replace(' has-text-danger has-background-danger-light', '')
    if (typeof validator === 'function') {
        isValid = validator(event)
    } else {
        isValid = validator
    }
    if (!isValid) {
        className += ' has-text-danger has-background-danger-light'
    }
    event.target.className = className
}

// Generates a unique name in a given entities list
const generateUniqueEntityName = (originalName, entitiesList, isCopy, divider = ' ') => {
    if (!originalName) {
        originalName = `new${divider}entity`
    }
    let name_prefix = ''
    if (isCopy) {
        name_prefix = `copy${divider}of${divider}`
    }
    let new_name = `${name_prefix}${originalName}`
    let counter = 1
    while (entitiesList.includes(new_name)) {
        counter++
        new_name = `${name_prefix}${originalName}(${counter})`
    }
    return new_name
}

// Download data as file

const downloadFile = (fileName, fileType, data) => {
    // Check if file type can be downloaded
    const recognizedDownloadFileTypes = ['json']
    if (!recognizedDownloadFileTypes.includes(fileType)) {
        console.log('Unable to download file, unknown file type')
        return
    }
    // Create downloadable content based on file type
    let content = null
    if (fileType === 'json') {
        content = JSON.stringify(data)
    }
    // Create anchor element with download data
    const blob = new Blob([content], {
        type: `application/${fileType}`
    })
    const link = document.createElement('a')
    link.href = window.URL.createObjectURL(blob)
    link.download = `${fileName}.${fileType}`
    // Click initiates the download
    link.click()
}

export default {
    name: 'Utils',
    validateInput,
    generateUniqueEntityName,
    downloadFile,
}
