import Utils from '../../assets/Utils'
import {afterEach, beforeEach, describe, expect, jest, test} from '@jest/globals'
import * as bulmaToast from 'bulma-toast'
import {Options} from 'bulma-toast'
import axios from 'axios'

describe('Utils.ts', () => {
  describe('generateUniqueEntityName function', () => {
    let entities: string[]
    let initialName: string
    beforeEach(() => {
      initialName = 'entityName'
      entities = [initialName]
    })

    test('should generate name for new entity', async () => {
      const wantedName = `new entity`

      const actualName = Utils.generateUniqueEntityName(null, entities)
      expect(actualName).toEqual(wantedName)
    })

    test('should generate name for new entity with given divider', async () => {
      const wantedName = `new___entity`

      const actualName = Utils.generateUniqueEntityName(null, entities, false, '___')
      expect(actualName).toEqual(wantedName)
    })

    test('should generate name for copied entity with given divider', async () => {
      const wantedName = `copy__of__${initialName}`

      const actualName = Utils.generateUniqueEntityName(initialName, entities, true, '__')
      expect(actualName).toEqual(wantedName)
    })

    test('should generate name for copied entity', async () => {
      const wantedName = `copy of ${initialName}`

      const actualName = Utils.generateUniqueEntityName(initialName, entities, true)
      expect(actualName).toEqual(wantedName)
    })

    test('should generate name for copied entity if copy exists in list', async () => {
      const firstCopyName = `copy of ${initialName}`
      const wantedName = `copy of ${initialName}(2)`
      entities.push(firstCopyName)

      const actualName = Utils.generateUniqueEntityName(initialName, entities, true)
      expect(actualName).toEqual(wantedName)
    })

    test('should generate name for copied entity if copy exists in list X times', async () => {
      const firstCopyName = `copy of ${initialName}`
      const wantedName = `copy of ${initialName}(5)`
      entities.push(firstCopyName, `${firstCopyName}(2)`, `${firstCopyName}(3)`, `${firstCopyName}(4)`)

      const actualName = Utils.generateUniqueEntityName(initialName, entities, true)
      expect(actualName).toEqual(wantedName)
    })
  })

  describe('validateInput function', () => {
    let event: any
    let validator
    const validValidator = () => {
      return true
    }
    const invalidValidator = () => {
      return false
    }
    beforeEach(() => {
      event = {
        target: {
          className: 'is-big is-bird is-yellow',
        },
        type: 'input',
      }
    })

    test('should handle boolean validator', async () => {
      validator = false
      Utils.validateInput(event, validator)
      expect(event.target.className).toContain('has-text-danger')
      expect(event.target.className).toContain('has-background-danger-light')
    })

    test('should handle functional validator', async () => {
      validator = () => false
      Utils.validateInput(event, validator)
      expect(event.target.className).toContain('has-text-danger')
      expect(event.target.className).toContain('has-background-danger-light')
    })

    test('should pass event to functional validator', async () => {
      validator = (e: Event) => {
        return e.type === 'click'
      }
      Utils.validateInput(event, validator)
      expect(event.target.className).toContain('has-text-danger')
      expect(event.target.className).toContain('has-background-danger-light')
    })

    test('should add classes to invalid input', async () => {
      validator = invalidValidator
      Utils.validateInput(event, validator)
      expect(event.target.className).toContain('has-text-danger')
      expect(event.target.className).toContain('has-background-danger-light')
    })

    test('should remove classes from valid input', async () => {
      validator = invalidValidator
      Utils.validateInput(event, validator)
      validator = validValidator
      Utils.validateInput(event, validator)
      expect(event.target.className).not.toContain('has-text-danger')
      expect(event.target.className).not.toContain('has-background-danger-light')
    })

    test('should not remove unrelated classes while validating', async () => {
      validator = validValidator
      Utils.validateInput(event, validator)
      expect(event.target.className).toContain('is-big')
      expect(event.target.className).toContain('is-bird')
      expect(event.target.className).toContain('is-yellow')
      validator = invalidValidator
      Utils.validateInput(event, validator)
      expect(event.target.className).toContain('is-big')
      expect(event.target.className).toContain('is-bird')
      expect(event.target.className).toContain('is-yellow')
    })
  })

  describe('clearInputValidationClasses function', () => {
    let event: any
    let validator
    const invalidValidator = () => {
      return false
    }
    beforeEach(() => {
      event = {
        target: {
          className: 'is-big is-bird is-yellow',
        },
        type: 'input',
      }
    })

    test('should remove classes from valid input element', async () => {
      validator = invalidValidator
      Utils.validateInput(event, validator)
      Utils.clearInputValidationClasses(event.target as HTMLInputElement)
      expect(event.target.className).not.toContain('has-text-danger')
      expect(event.target.className).not.toContain('has-background-danger-light')
    })

    test('should not remove unrelated classes while validating', async () => {
      validator = invalidValidator()
      Utils.validateInput(event, validator)
      expect(event.target.className).toContain('is-big')
      expect(event.target.className).toContain('is-bird')
      expect(event.target.className).toContain('is-yellow')
      Utils.clearInputValidationClasses(event.target as HTMLInputElement)
      expect(event.target.className).toContain('is-big')
      expect(event.target.className).toContain('is-bird')
      expect(event.target.className).toContain('is-yellow')
    })
  })

  describe('downloadFile function', () => {
    let fileName: string
    let fileType: string
    let data: Object
    beforeEach(() => {
      fileName = 'test'
      fileType = 'json'
      data = {
        'foo': 'bar',
      }
      // This line makes it so the new <a> tag is not really an <a> tag and would not try to link to anything
      // @ts-ignore
      document.createElement = () => {
        return {
          click: () => {
          },
        }
      }
    })

    test('should not throw errors if given valid input', (done) => {
      try {
        Utils.downloadFile(fileName, fileType, data)
        done()
      } catch (err) {
        expect(err).not.toBeDefined()
        done()
      }
    })

    test('should not log errors if given valid input', (done) => {
      const originalLog = console.log
      const consoleOutput: string[] = []
      console.log = (output: string) => consoleOutput.push(output)
      Utils.downloadFile(fileName, fileType, data)
      // allow all requests to finish
      setImmediate(() => {
        expect(consoleOutput).toEqual([])
        console.log = originalLog
        done()
      })
    })

    test('should log message when receiving unknown file type', (done) => {
      const originalLog = console.log
      const consoleOutput: string[] = []
      console.log = (output: string) => consoleOutput.push(output)
      Utils.downloadFile(fileName, 'weird string', data)
      // allow all requests to finish
      setImmediate(() => {
        expect(consoleOutput).toContain(`Unable to download file, unknown file type`)
        console.log = originalLog
        done()
      })
    })
  })

  describe('toast function', () => {
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

    test('should send success toast', () => {
      Utils.successToast(successMessage)
      expect(toastOutput[0].message).toContain(successMessage)
      expect(toastOutput[0].type).toContain(successMessageClass)
    })

    test('should send failure toast', () => {
      jest.spyOn(axios, 'get').mockImplementationOnce(() => Promise.reject(new Error()))
      Utils.failureToast(failureMessage)
      expect(toastOutput[0].message).toContain(failureMessage)
      expect(toastOutput[0].type).toContain(failureMessageClass)
    })
  })
})
