import {afterEach, beforeEach, describe, expect, jest, test} from '@jest/globals'
import DateTimeUtils from '@/assets/DateTimeUtils'

describe('DateTimeUtils.ts', () => {
  describe('isoToNowCuriefenseFormat function', () => {
    let date: Date

    const buildTests = (type: 'Date' | 'string') => {
      let callFunc: any
      if (type === 'Date') {
        callFunc = (date: Date) => {
          return DateTimeUtils.isoToNowCuriefenseFormat(date)
        }
      }
      if (type === 'string') {
        callFunc = (date: Date) => {
          return DateTimeUtils.isoToNowCuriefenseFormat(date.toISOString())
        }
      }
      describe(`${type} type input`, () => {
        describe('Less than a minute', () => {
          test('should display correct message', async () => {
            const wantedTime = `Less than a minute ago`
            date = new Date()
            date.setSeconds(date.getSeconds() - 30)

            const actualTime = callFunc(date)
            expect(actualTime).toEqual(wantedTime)
          })
        })

        describe('Between a minute and a day', () => {
          test('should display correct message for difference of a single minute', async () => {
            const wantedTime = `1 minute ago`
            date = new Date()
            date.setMinutes(date.getMinutes() - 1)

            const actualTime = callFunc(date)
            expect(actualTime).toEqual(wantedTime)
          })

          test('should display correct message for difference of multiple minutes', async () => {
            const wantedMinutesDiff = 5
            const wantedTime = `${wantedMinutesDiff} minutes ago`
            date = new Date()
            date.setMinutes(date.getMinutes() - wantedMinutesDiff)

            const actualTime = callFunc(date)
            expect(actualTime).toEqual(wantedTime)
          })

          test('should display correct message for difference of a single hour', async () => {
            const wantedTime = `1 hour ago`
            date = new Date()
            date.setHours(date.getHours() - 1)

            const actualTime = callFunc(date)
            expect(actualTime).toEqual(wantedTime)
          })

          test('should display correct message for difference of multiple hours', async () => {
            const wantedHoursDiff = 5
            const wantedTime = `${wantedHoursDiff} hours ago`
            date = new Date()
            date.setHours(date.getHours() - wantedHoursDiff)

            const actualTime = callFunc(date)
            expect(actualTime).toEqual(wantedTime)
          })

          test('should display correct message for difference of a single hour single minute', async () => {
            const wantedTime = `1 hour 1 minute ago`
            date = new Date()
            date.setHours(date.getHours() - 1)
            date.setMinutes(date.getMinutes() - 1)

            const actualTime = callFunc(date)
            expect(actualTime).toEqual(wantedTime)
          })

          test('should display correct message for difference of a single hour multiple minutes', async () => {
            const wantedMinutesDiff = 3
            const wantedTime = `1 hour ${wantedMinutesDiff} minutes ago`
            date = new Date()
            date.setHours(date.getHours() - 1)
            date.setMinutes(date.getMinutes() - wantedMinutesDiff)

            const actualTime = callFunc(date)
            expect(actualTime).toEqual(wantedTime)
          })

          test('should display correct message for difference of a multiple hours single minute', async () => {
            const wantedHoursDiff = 7
            const wantedTime = `${wantedHoursDiff} hours 1 minute ago`
            date = new Date()
            date.setHours(date.getHours() - wantedHoursDiff)
            date.setMinutes(date.getMinutes() - 1)

            const actualTime = callFunc(date)
            expect(actualTime).toEqual(wantedTime)
          })

          test('should display correct message for difference of a multiple hours multiple minutes', async () => {
            const wantedHoursDiff = 2
            const wantedMinutesDiff = 3
            const wantedTime = `${wantedHoursDiff} hours ${wantedMinutesDiff} minutes ago`
            date = new Date()
            date.setHours(date.getHours() - wantedHoursDiff)
            date.setMinutes(date.getMinutes() - wantedMinutesDiff)

            const actualTime = callFunc(date)
            expect(actualTime).toEqual(wantedTime)
          })
        })

        describe('Between a day and three months', () => {
          let month: string
          let day: number | string
          let hours: number | string
          let minutes: number | string
          const calcWantedTime = () => {
            month = date.toLocaleString('default', {month: 'short'})
            day = date.getDate()
            if (day > 3 && day < 21) {
              day = `${day}th`
            } else {
              switch (day % 10) {
                case 1:
                  day = `${day}st`
                  break
                case 2:
                  day = `${day}nd`
                  break
                case 3:
                  day = `${day}rd`
                  break
                default:
                  day = `${day}th`
              }
            }
            hours = date.getHours()
            if (hours < 10) {
              hours = `0${hours}`
            }
            minutes = date.getMinutes()
            if (minutes < 10) {
              minutes = `0${minutes}`
            }
            return `${day} ${month}, ${hours}:${minutes}`
          }

          test('should display correct message when two days old', async () => {
            date = new Date()
            date.setDate(date.getDate() - 2)
            const wantedTime = calcWantedTime()

            const actualTime = callFunc(date)
            expect(actualTime).toEqual(wantedTime)
          })

          for (let i = 0; i < 28; i++) {
            test(`should display correct message when ${i} of month (date ordinal)`, async () => {
              date = new Date()
              date.setMonth(date.getMonth() - 1)
              date.setDate(i)
              const wantedTime = calcWantedTime()

              const actualTime = callFunc(date)
              expect(actualTime).toEqual(wantedTime)
            })
          }

          test('should display correct message when hours are single digit', async () => {
            date = new Date()
            date.setDate(date.getDate() - 3)
            date.setHours(2)
            const wantedTime = calcWantedTime()

            const actualTime = callFunc(date)
            expect(actualTime).toEqual(wantedTime)
          })

          test('should display correct message when hours are multi digit', async () => {
            date = new Date()
            date.setDate(date.getDate() - 3)
            date.setHours(14)
            const wantedTime = calcWantedTime()

            const actualTime = callFunc(date)
            expect(actualTime).toEqual(wantedTime)
          })

          test('should display correct message when minutes are single digit', async () => {
            date = new Date()
            date.setDate(date.getDate() - 3)
            date.setMinutes(2)
            const wantedTime = calcWantedTime()

            const actualTime = callFunc(date)
            expect(actualTime).toEqual(wantedTime)
          })

          test('should display correct message when minutes are multi digit', async () => {
            date = new Date()
            date.setDate(date.getDate() - 3)
            date.setMinutes(37)
            const wantedTime = calcWantedTime()

            const actualTime = callFunc(date)
            expect(actualTime).toEqual(wantedTime)
          })
        })

        describe('Over three months', () => {
          let original: (date: string | Date) => string
          let spy: any
          beforeEach(() => {
            original = DateTimeUtils.isoToNowFullCuriefenseFormat
            spy = DateTimeUtils.isoToNowFullCuriefenseFormat = jest.fn()
          })
          afterEach(() => {
            DateTimeUtils.isoToNowFullCuriefenseFormat = original
          })

          test('should call isoToNowFullCuriefenseFormat function', async () => {
            date = new Date()
            date.setMonth(date.getMonth() - 24)

            callFunc(date)
            expect(spy).toHaveBeenCalledWith(date)
          })
        })
      })
    }

    buildTests('Date')
    buildTests('string')
  })

  describe('isoToNowFullCuriefenseFormat function', () => {
    let date: Date

    const buildTests = (type: 'Date' | 'string') => {
      let callFunc: any
      if (type === 'Date') {
        callFunc = (date: Date) => {
          return DateTimeUtils.isoToNowFullCuriefenseFormat(date)
        }
      }
      if (type === 'string') {
        callFunc = (date: Date) => {
          return DateTimeUtils.isoToNowFullCuriefenseFormat(date.toISOString())
        }
      }
      describe(`${type} type input`, () => {
        let year: number
        let month: string
        let day: number | string
        let hours: number | string
        let minutes: number | string
        let seconds: number | string
        const calcWantedTime = () => {
          year = date.getFullYear()
          month = date.toLocaleString('default', {month: 'short'})
          day = date.getDate()
          if (day > 3 && day < 21) {
            day = `${day}th`
          } else {
            switch (day % 10) {
              case 1:
                day = `${day}st`
                break
              case 2:
                day = `${day}nd`
                break
              case 3:
                day = `${day}rd`
                break
              default:
                day = `${day}th`
            }
          }
          hours = date.getHours()
          if (hours < 10) {
            hours = `0${hours}`
          }
          minutes = date.getMinutes()
          if (minutes < 10) {
            minutes = `0${minutes}`
          }
          seconds = date.getSeconds()
          if (seconds < 10) {
            seconds = `0${seconds}`
          }
          return `${day} ${month} ${year}, ${hours}:${minutes}:${seconds}`
        }

        test('should display correct message when two days old', async () => {
          date = new Date()
          date.setDate(date.getDate() - 2)
          const wantedTime = calcWantedTime()

          const actualTime = callFunc(date)
          expect(actualTime).toEqual(wantedTime)
        })

        test('should display correct message when date is now', async () => {
          date = new Date()
          const wantedTime = calcWantedTime()

          const actualTime = callFunc(date)
          expect(actualTime).toEqual(wantedTime)
        })

        for (let i = 0; i < 28; i++) {
          test(`should display correct message when ${i} of month (date ordinal)`, async () => {
            date = new Date()
            date.setMonth(date.getMonth() - 1)
            date.setDate(i)
            const wantedTime = calcWantedTime()

            const actualTime = callFunc(date)
            expect(actualTime).toEqual(wantedTime)
          })
        }

        test('should display correct message when hours are single digit', async () => {
          date = new Date()
          date.setDate(date.getDate() - 3)
          date.setHours(2)
          const wantedTime = calcWantedTime()

          const actualTime = callFunc(date)
          expect(actualTime).toEqual(wantedTime)
        })

        test('should display correct message when hours are multi digit', async () => {
          date = new Date()
          date.setDate(date.getDate() - 3)
          date.setHours(14)
          const wantedTime = calcWantedTime()

          const actualTime = callFunc(date)
          expect(actualTime).toEqual(wantedTime)
        })

        test('should display correct message when minutes are single digit', async () => {
          date = new Date()
          date.setDate(date.getDate() - 3)
          date.setMinutes(2)
          const wantedTime = calcWantedTime()

          const actualTime = callFunc(date)
          expect(actualTime).toEqual(wantedTime)
        })

        test('should display correct message when minutes are multi digit', async () => {
          date = new Date()
          date.setDate(date.getDate() - 3)
          date.setMinutes(37)
          const wantedTime = calcWantedTime()

          const actualTime = callFunc(date)
          expect(actualTime).toEqual(wantedTime)
        })

        test('should display correct message when seconds are single digit', async () => {
          date = new Date()
          date.setDate(date.getDate() - 3)
          date.setSeconds(8)
          const wantedTime = calcWantedTime()

          const actualTime = callFunc(date)
          expect(actualTime).toEqual(wantedTime)
        })

        test('should display correct message when seconds are multi digit', async () => {
          date = new Date()
          date.setDate(date.getDate() - 3)
          date.setSeconds(20)
          const wantedTime = calcWantedTime()

          const actualTime = callFunc(date)
          expect(actualTime).toEqual(wantedTime)
        })
      })
    }

    buildTests('Date')
    buildTests('string')
  })
})
