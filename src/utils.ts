let STOP_LOOP = true

export const setLoop = (fn: (delta: number) => void) => {
  STOP_LOOP = false
  let last = window.performance.now()
  const loop = () => {
    let now = window.performance.now()
    fn(now - last)
    last = now
    if (!STOP_LOOP) {
      requestAnimationFrame(loop)
    }
  }
  requestAnimationFrame(loop)
}

export const stopLoop = () => {
  STOP_LOOP = true
}

export const arr = (len: number) => Array.from({ length: len }).fill(null)

export const easeOutQuart = (x: number) => {
  return 1 - Math.pow(1 - x, 4)
}

export const easeInOutSine = (x: number) => {
  return -(Math.cos(Math.PI * x) - 1) / 2
}


type AnimeArguments = {
  start: number
  end: number
  duration: number
  easeFn?: (x: number) => number
  update?: (value: number, stepValue: number) => void
  complete?: () => void
  canceled?: () => void
}

const isOverEnd = (start: number, end: number, value: number) => {
  return (start < end && value > end) || (start >= end && value < end)
}

export function animate({
  start = 0,
  end = 0,
  duration = 300,
  easeFn = x => 1 - (1 - x) ** 3,
  update = () => null,
  complete = () => null,
  canceled = () => null,
}: AnimeArguments) {
  if (duration === 0) {
    update(end, 0)
    complete()
    return () => null
  }

  let startTime = 0
  let value = 0
  let lastValue = start
  let lastRFA = 0
  let cancel = false

  const step = () => {
    const now = window.performance.now()
    if (startTime === 0) {
      startTime = now
    }

    const elapsed = Math.min(now - startTime, duration)

    value = start + (end - start) * easeFn(elapsed / duration)

    if (isOverEnd(start, end, value)) {
      value = end
    }

    let stepValue = value - lastValue
    lastValue = value

    if (Math.abs(stepValue) < 0.1 && stepValue !== 0) {
      value = end
      stepValue = value - lastValue
    }

    update(Number(Number(value).toFixed(6)), Number(Number(stepValue).toFixed(6)))

    if (value !== end && !cancel) {
      cancelAnimationFrame(lastRFA)
      lastRFA = requestAnimationFrame(step)
    } else if (!cancel) {
      complete()
    } else {
      canceled()
    }
  }

  lastRFA = requestAnimationFrame(step)

  return () => {
    cancel = true
  }
}