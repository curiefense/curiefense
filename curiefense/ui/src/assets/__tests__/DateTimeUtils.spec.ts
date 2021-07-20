import {beforeEach, describe, expect, test} from '@jest/globals'
import DateTimeUtils from '@/assets/DateTimeUtils'

describe('DateTimeUtils.ts', () => {
  let date: Date
  beforeEach(() => {
    date = new Date()
  })

  const buildTests = (type: 'Date' | 'string') => {
    const callFunc: Function = (date: Date) => DateTimeUtils.isoToNowCuriefenseFormat(
      type === 'Date' ? date : date.toISOString(),
    )

    describe(`${type} type input`, () => {
      const parseDate = () => {
        const year = date.getFullYear()
        const day = date.getDate()
        const month = date.toLocaleString('default', {month: '2-digit'})
        const time = date.toLocaleString('default', {hour: '2-digit', minute: '2-digit'})
        const [hoursMinutes, ampm] = time.split(' ')
        return {year, month, day, hoursMinutes, ampm}
      }
      const calcWantedDate = () => {
        const {year, month, day} = parseDate()
        return `${year}-${month}-${day}`
      }
      const calcWantedTime = () => {
        const {hoursMinutes, ampm} = parseDate()
        return `${hoursMinutes}${ampm.toLowerCase()}`
      }
      const calcWantedDatetime = () => {
        const {year, month, day, hoursMinutes, ampm} = parseDate()
        return `${year}-${month}-${day} ${hoursMinutes}${ampm.toLowerCase()}`
      }

      test('should display correct message when two days old', async () => {
        date.setDate(date.getDate() - 2)
        const wantedTime = calcWantedDatetime()
        const actualTime = callFunc(date)
        expect(actualTime).toEqual(wantedTime)
      })

      test('should display correct date when two days old', async () => {
        date.setDate(date.getDate() - 2)
        const wantedTime = calcWantedDate()
        const actualTime = DateTimeUtils.isoToNowDateCuriefenseFormat(date)
        expect(actualTime).toEqual(wantedTime)
      })

      test('should display correct message when hours are single digit', async () => {
        date.setDate(date.getDate() - 3)
        date.setHours(2)
        const wantedTime = calcWantedDatetime()
        const actualTime = callFunc(date)
        expect(actualTime).toEqual(wantedTime)
      })

      test('should display correct time', async () => {
        date.setDate(date.getDate() - 3)
        date.setHours(2)
        const wantedTime = calcWantedTime()
        const actualTime = DateTimeUtils.isoToNowTimeCuriefenseFormat(date)
        expect(actualTime).toEqual(wantedTime)
      })

      test('should display correct message when hours are multi digit', async () => {
        date.setDate(date.getDate() - 3)
        date.setHours(14)
        const wantedTime = calcWantedDatetime()
        const actualTime = callFunc(date)
        expect(actualTime).toEqual(wantedTime)
      })

      test('should display correct message when minutes are single digit', async () => {
        date.setDate(date.getDate() - 3)
        date.setMinutes(2)
        const wantedTime = calcWantedDatetime()
        const actualTime = callFunc(date)
        expect(actualTime).toEqual(wantedTime)
      })

      test('should display correct message when minutes are multi digit', async () => {
        date.setDate(date.getDate() - 3)
        date.setMinutes(37)
        const wantedTime = calcWantedDatetime()
        const actualTime = callFunc(date)
        expect(actualTime).toEqual(wantedTime)
      })
    })
  }

  buildTests('Date')
  buildTests('string')
})
