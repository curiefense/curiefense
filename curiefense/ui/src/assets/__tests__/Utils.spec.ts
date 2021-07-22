import Utils from '../../assets/Utils'
import {afterEach, beforeEach, describe, expect, jest, test} from '@jest/globals'
import * as bulmaToast from 'bulma-toast'
import {Options} from 'bulma-toast'
import axios from 'axios'
import {JSDOM} from 'jsdom'

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
    let toastOutput: Options[] = []
    beforeEach(() => {
      toastOutput = []
      jest.spyOn(bulmaToast, 'toast').mockImplementation((output: Options) => {
        toastOutput.push(output)
      })
      const dom = new JSDOM()
      Object.defineProperty(global, 'document', {
        writable: true,
        value: dom.window.document,
      })
      Object.defineProperty(global, 'window', {
        writable: true,
        value: dom.window,
      })
    })
    afterEach(() => {
      jest.clearAllMocks()
    })

    test('should display success toast', () => {
      const successMessage = 'yay we did it!'
      const successMessageClass = 'is-success'
      Utils.toast(successMessage, 'is-success')
      expect(toastOutput[0].message).toContain(successMessage)
      expect(toastOutput[0].type).toContain(successMessageClass)
    })

    test('should display failure toast', () => {
      const failureMessage = 'oops, something went wrong'
      const failureMessageClass = 'is-danger'
      jest.spyOn(axios, 'get').mockImplementationOnce(() => Promise.reject(new Error()))
      Utils.toast(failureMessage, 'is-danger')
      expect(toastOutput[0].message).toContain(failureMessage)
      expect(toastOutput[0].type).toContain(failureMessageClass)
    })

    test('should display info toast', () => {
      const infoMessage = 'Notice me'
      const infoMessageClass = 'is-info'
      Utils.toast(infoMessage, 'is-info')
      expect(toastOutput[0].message).toContain(infoMessage)
      expect(toastOutput[0].type).toContain(infoMessageClass)
    })

    test('should display toast html element message', () => {
      const message = 'message text goes here'
      const textElement = document.createElement('span')
      textElement.innerText = message
      const successMessageClass = 'is-success'
      Utils.toast(textElement, 'is-success')
      expect(toastOutput[0].message).toEqual(textElement)
      expect((toastOutput[0].message as HTMLElement).innerText).toEqual(message)
      expect(toastOutput[0].type).toContain(successMessageClass)
    })

    test('should display toast message correctly if given undo function', () => {
      const successMessage = 'yay we did it!'
      const undoFunction = () => {
        return true
      }
      Utils.toast(successMessage, 'is-success', undoFunction)
      const toastMessageElement = (toastOutput[0].message as HTMLElement).getElementsByTagName('span')[0]
      expect(toastMessageElement.innerText).toContain(successMessage)
    })

    test('should display toast html element message correctly if given undo function', () => {
      const undoFunction = () => {
        return true
      }
      const message = 'message text goes here'
      const textElement = document.createElement('span')
      textElement.innerText = message
      const successMessageClass = 'is-success'
      Utils.toast(textElement, 'is-success', undoFunction)
      const toastMessageElement = (toastOutput[0].message as HTMLElement).getElementsByTagName('span')[0]
      expect(toastMessageElement).toEqual(textElement)
      expect((toastMessageElement).innerText).toEqual(message)
      expect(toastOutput[0].type).toContain(successMessageClass)
    })


    test('should display toast undo message if given undo function', () => {
      const undoFunction = () => {
        return true
      }
      const undoMessage = 'To undo this action, click here.'
      Utils.toast('ok', 'is-success', undoFunction)
      const toastUndoPrefixElement = (toastOutput[0].message as HTMLElement).getElementsByTagName('span')[1]
      const toastUndoAnchorElement = (toastOutput[0].message as HTMLElement).getElementsByTagName('a')[0]
      const toastUndoSuffixElement = (toastOutput[0].message as HTMLElement).getElementsByTagName('span')[2]
      const actualUndoText = toastUndoPrefixElement.innerText +
        toastUndoAnchorElement.innerText +
        toastUndoSuffixElement.innerText
      expect(actualUndoText).toContain(undoMessage)
    })

    test('should display toast with an undo link with an anchor tag', () => {
      const undoFunction = () => {
        return true
      }
      const undoAnchorElement = document.createElement('a')
      undoAnchorElement.onclick = undoFunction
      undoAnchorElement.innerText = 'here'
      Utils.toast('ok', 'is-success', undoFunction)
      const toastUndoAnchorElement = (toastOutput[0].message as HTMLElement).getElementsByTagName('a')[0]
      expect(toastUndoAnchorElement).toEqual(undoAnchorElement)
      expect(toastUndoAnchorElement.onclick).toEqual(undoAnchorElement.onclick)
    })

    test('should call undo function if anchor is clicked', (done) => {
      const undoFunction = () => {
        done()
      }
      const undoAnchorElement = document.createElement('a')
      undoAnchorElement.onclick = undoFunction
      undoAnchorElement.innerText = 'here'
      Utils.toast('ok', 'is-success', undoFunction)
      const toastUndoAnchorElement = (toastOutput[0].message as HTMLElement).getElementsByTagName('a')[0]
      toastUndoAnchorElement.click()
    })
  })

  describe('removeExtraWhitespaces function', () => {
    test('should ignore multiple spaces when emitting tag-changed', async () => {
      const inputValue = 'some-value                    some-other-value'
      const wantedValue = 'some-value some-other-value'
      const actualResult = Utils.removeExtraWhitespaces(inputValue)
      expect(actualResult).toEqual(wantedValue)
    })

    test('should ignore tabs when emitting tag-changed', async () => {
      const inputValue = 'some-value             some-other-value'
      const wantedValue = 'some-value some-other-value'
      const actualResult = Utils.removeExtraWhitespaces(inputValue)
      expect(actualResult).toEqual(wantedValue)
    })

    test('should ignore new lines when emitting tag-changed', async () => {
      const inputValue = 'some-value \n\n\n some-other-value'
      const wantedValue = 'some-value some-other-value'
      const actualResult = Utils.removeExtraWhitespaces(inputValue)
      expect(actualResult).toEqual(wantedValue)
    })

    test('should replace whitespace at end of string without trimming it', async () => {
      const inputValue = 'some-value some-other-value       \n\n      '
      const wantedValue = 'some-value some-other-value '
      const actualResult = Utils.removeExtraWhitespaces(inputValue)
      expect(actualResult).toEqual(wantedValue)
    })

    test('should replace whitespace at start of string without trimming it', async () => {
      const inputValue = '       \n\n      some-value some-other-value'
      const wantedValue = ' some-value some-other-value'
      const actualResult = Utils.removeExtraWhitespaces(inputValue)
      expect(actualResult).toEqual(wantedValue)
    })
  })
})
