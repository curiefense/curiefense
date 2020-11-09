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

export default {
    name: 'Utils',
    validateInput,
    generateUniqueEntityName
}
