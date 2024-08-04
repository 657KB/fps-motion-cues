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