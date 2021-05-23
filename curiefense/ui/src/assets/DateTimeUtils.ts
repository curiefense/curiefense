// Helper function to add date ordinal
import DateTimeUtils from '@/assets/DateTimeUtils'

const nth = function(day: number) {
  if (day > 3 && day < 21) return 'th'
  switch (day % 10) {
    case 1:
      return 'st'
    case 2:
      return 'nd'
    case 3:
      return 'rd'
    default:
      return 'th'
  }
}

// Formats the received ISO date into a more user-friendly format
const isoToNowCuriefenseFormat = (date: string | Date) => {
  if (typeof date === 'string') {
    date = new Date(date)
  }
  const currentDate = new Date()
  // @ts-ignore
  const dateDiff = currentDate - date
  const minutesDiff = dateDiff / 6e4
  const hoursDiff = dateDiff / 36e5
  const daysDiff = dateDiff / 864e5
  // if less than 1 minute ago
  if (minutesDiff < 1) {
    return 'Less than a minute ago'
  }
  // if less than 24 hours ago
  if (hoursDiff < 24) {
    let returnString = ''
    const flooredHoursDiff = Math.floor(hoursDiff)
    if (flooredHoursDiff > 0) {
      returnString += `${flooredHoursDiff} hour`
      if (flooredHoursDiff > 1) {
        returnString += 's'
      }
      returnString += ' '
    }
    const flooredMinutesDiff = Math.floor(minutesDiff)
    if (flooredMinutesDiff % 60 > 0) {
      returnString += `${flooredMinutesDiff % 60} minute`
      if (flooredMinutesDiff % 60 > 1) {
        returnString += 's'
      }
      returnString += ' '
    }
    returnString += 'ago'
    return returnString
  }
  // if less than 1 year ago
  if (daysDiff < 180) {
    const month = date.toLocaleString('default', {month: 'short'})
    const day = date.getDate()
    const hours = date.getHours() < 10 ? `0${date.getHours()}` : date.getHours()
    const minutes = date.getMinutes() < 10 ? `0${date.getMinutes()}` : date.getMinutes()
    return `${day}${nth(day)} ${month}, ${hours}:${minutes}`
  }
  return DateTimeUtils.isoToNowFullCuriefenseFormat(date)
}

// Full date formatted display
const isoToNowFullCuriefenseFormat = (date: string | Date) => {
  if (typeof date === 'string') {
    date = new Date(date)
  }
  const year = date.getFullYear()
  const month = date.toLocaleString('default', {month: 'short'})
  const day = date.getDate()
  const hours = date.getHours() < 10 ? `0${date.getHours()}` : date.getHours()
  const minutes = date.getMinutes() < 10 ? `0${date.getMinutes()}` : date.getMinutes()
  const seconds = date.getSeconds() < 10 ? `0${date.getSeconds()}` : date.getSeconds()
  return `${day}${nth(day)} ${month} ${year}, ${hours}:${minutes}:${seconds}`
}

export default {
  name: 'DateTimeUtils',
  isoToNowCuriefenseFormat,
  isoToNowFullCuriefenseFormat,
}
