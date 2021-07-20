// Formats the received ISO date into Curiefense format YYYY-MM-dd HH:mm+ampm
const parseDate = (date: string | Date) => {
  if (typeof date === 'string') {
    date = new Date(date)
  }
  const year = date.getFullYear()
  const day = date.getDate()
  const month = date.toLocaleString('default', {month: '2-digit'})
  const time = date.toLocaleString('default', {hour: '2-digit', minute: '2-digit'})
  const [hoursMinutes, ampm] = time.split(' ')
  return {year, month, day, hoursMinutes, ampm}
}

const isoToNowDateCuriefenseFormat = (date: string | Date) => {
  const {year, month, day} = parseDate(date)
  return `${year}-${month}-${day}`
}

const isoToNowTimeCuriefenseFormat = (date: string | Date) => {
  const {hoursMinutes, ampm} = parseDate(date)
  return `${hoursMinutes}${ampm.toLowerCase()}`
}

// Full date formatted display
const isoToNowCuriefenseFormat = (date: string | Date) => {
  const {year, month, day, hoursMinutes, ampm} = parseDate(date)
  return `${year}-${month}-${day} ${hoursMinutes}${ampm.toLowerCase()}`
}

export default {
  name: 'DateTimeUtils',
  isoToNowCuriefenseFormat,
  isoToNowTimeCuriefenseFormat,
  isoToNowDateCuriefenseFormat,
}
