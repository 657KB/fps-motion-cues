export let STOP_LOOP = true

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

export const arr = (len: number) => Array.from({ length: len }).fill(null)

export const easeOutQuart = (x: number) => {
  return 1 - Math.pow(1 - x, 4)
}
export const easeInOutSine = (x: number) => {
  return -(Math.cos(Math.PI * x) - 1) / 2
}