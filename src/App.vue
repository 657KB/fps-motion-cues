<script setup lang="ts">
import { onMounted, onUnmounted, ref } from 'vue'
import { arr, setLoop } from './utils'
// import { listen } from '@tauri-apps/api/event'

// type Key = { code: string }
// type MouseMove = { coords: [number, number] }

const canvas = ref<HTMLCanvasElement | null>(null)

const position = { x: 0, y: 0 }

// The direction in which the player moves in the 3d coordinate axis
const moveDirection = { x: 0, y: 0, z: 0 }
// The direction of the mouse movement perspective
const viewDirection = { x: 0, y: 0 }

const moveSpeed = 1
const jumpSpeed = 1

// const keydown = listen<Key>('key_down', event => {
//   const { code } = event.payload
//   if (code === 'W' || code === 'Up') {
//     moveDirection.y -= 1
//   } else if (code === 'A' || code === 'Left') {
//     moveDirection.x += 1
//   } else if (code === 'S' || code === 'Down') {
//     moveDirection.y += 1
//   } else if (code === 'D' || code === 'Right') {
//     moveDirection.x -= 1
//   } else if (code === 'Space') {
//     moveDirection.z = 1
//   }
// })

// const keyup = listen<Key>('key_up', event => {
//   const { code } = event.payload
//   if (code === 'W' || code === 'Up') {
//     moveDirection.y += 1
//   } else if (code === 'A' || code === 'Left') {
//     moveDirection.x -= 1
//   } else if (code === 'S' || code === 'Down') {
//     moveDirection.y -= 1
//   } else if (code === 'D' || code === 'Right') {
//     moveDirection.x += 1
//   } else if (code === 'Space') {
//     moveDirection.z = 0
//   }
// })

// const mousemove = listen<MouseMove>('mouse_move', event => {
//   // console.log(event.payload.coords)
// })

const update = (delta: number) => {
  if (canvas.value === null) return

  const ctx = canvas.value.getContext('2d')
  const w = canvas.value.width
  const h = canvas.value.height

  if (ctx === null) return

  ctx.clearRect(0, 0, w, h)
  ctx.fillStyle = '#000'

  position.x += (moveSpeed / 10) * delta * moveDirection.x

  canvas.value.style.transform = `scale(calc(1 + ${moveDirection.y * 0.1}))`

  const maxDistanceX = w / 4
  const rightStartPosX = w - maxDistanceX
  const maxLeftRadiusPosX = maxDistanceX / 2
  const maxRightRadiusPosX = rightStartPosX + maxDistanceX / 2
  const maxRadius = 12

  const drawSpot = (x: number, y: number, r: number) => {
    ctx.beginPath()
    ctx.arc(x, y, r, 0, Math.PI * 2, true)
    ctx.fill()
  }
  const leftProgress = (x: number) => 1 - Math.abs(Math.abs(x / maxLeftRadiusPosX) - 1)
  const rightProgress = (x: number) => 1 - Math.abs(Math.abs((w - x) / (w - maxRightRadiusPosX)) - 1)

  // left
  arr(5).forEach((_, index) => {
    const rawX = position.x % maxDistanceX
    const x = position.x < 0 ? maxDistanceX + rawX : rawX
    const y = h / 24 + (h / 5) * index
    drawSpot(x, y, maxRadius * leftProgress(x))
  })

  arr(3).forEach((_, index) => {
    const offsetX = maxDistanceX / 3
    const rawX = (position.x + offsetX) % maxDistanceX
    const x = position.x < -offsetX ? maxDistanceX + rawX % maxDistanceX : rawX
    const y = h / 6 + (h / 3) * index
    drawSpot(x, y, maxRadius * leftProgress(x))
  })

  arr(5).forEach((_, index) => {
    const offsetX = maxDistanceX * 2 / 3
    const rawX = (position.x + offsetX) % maxDistanceX
    const x = position.x < -offsetX ? maxDistanceX + rawX : rawX
    const y = h / 10 + (h / 5) * index
    drawSpot(x, y, maxRadius * leftProgress(x))
  })

  // right
  arr(5).forEach((_, index) => {
    const rawX = rightStartPosX + position.x % maxDistanceX
    const x = position.x + rightStartPosX < rightStartPosX ? maxDistanceX + rawX : rawX
    const y = h / 10 + (h / 5) * index
    drawSpot(x, y, maxRadius * rightProgress(x))
  })

  arr(3).forEach((_, index) => {
    const offsetX = maxDistanceX / 3
    const rawX = rightStartPosX + (position.x + offsetX) % maxDistanceX
    const x = position.x + rightStartPosX < rightStartPosX - offsetX ? maxDistanceX + rawX : rawX
    const y = h / 6 + (h / 3) * index
    drawSpot(x, y, maxRadius * 1.4 * rightProgress(x))
  })

  arr(5).forEach((_, index) => {
    const offsetX = maxDistanceX * 2 / 3
    const rawX = rightStartPosX + (position.x + offsetX) % maxDistanceX
    const x = position.x + rightStartPosX < rightStartPosX - offsetX ? maxDistanceX + rawX : rawX
    const y = h / 24 + (h / 5) * index
    drawSpot(x, y, maxRadius * rightProgress(x))
    ctx.fill()
  })
}

const resizeCanvas = () => {
  if (canvas.value !== null) {
    canvas.value.width = document.body.clientWidth
    canvas.value.height = document.body.clientHeight
  }
}

const handleKeyDown = (event: KeyboardEvent) => {
  const { code } = event
  console.log(event.code)
  if (code === 'KeyW') {
    moveDirection.y += 1
  } else if (code === 'KeyA') {
    moveDirection.x += 1
  } else if (code === 'KeyS') {
    moveDirection.y -= 1
  } else if (code === 'KeyD') {
    moveDirection.x -= 1
  } else if (code === 'Space') {
    moveDirection.z = 1
  }
  if (moveDirection.x > 1) {
    moveDirection.x = 1
  } else if (moveDirection.x < -1) {
    moveDirection.x = -1
  }
  if (moveDirection.y > 1) {
    moveDirection.y = 1
  } else if (moveDirection.y < -1) {
    moveDirection.y = -1
  }
}
const handleKeyUp = (event: KeyboardEvent) => {
  const { code } = event
  if (code === 'KeyW') {
    moveDirection.y -= 1
  } else if (code === 'KeyA') {
    moveDirection.x -= 1
  } else if (code === 'KeyS') {
    moveDirection.y += 1
  } else if (code === 'KeyD') {
    moveDirection.x += 1
  } else if (code === 'Space') {
    moveDirection.z = 1
  }
  if (moveDirection.x > 1) {
    moveDirection.x = 1
  } else if (moveDirection.x < -1) {
    moveDirection.x = -1
  }
  if (moveDirection.y > 1) {
    moveDirection.y = 1
  } else if (moveDirection.y < -1) {
    moveDirection.y = -1
  }
}

onMounted(() => {
  resizeCanvas()
  window.addEventListener('keydown', handleKeyDown)
  window.addEventListener('keyup', handleKeyUp)
  window.addEventListener('resize', resizeCanvas)
  setLoop(update)
  // update(0)
})

onUnmounted(() => {
  // keydown.then(unlisten => unlisten())
  // keyup.then(unlisten => unlisten())
  // mousemove.then(unlisten => unlisten())
  window.removeEventListener('keydown', handleKeyDown)
  window.removeEventListener('keyup', handleKeyUp)
  window.removeEventListener('resize', resizeCanvas)
})
</script>

<template>
  <div class="container">
    <canvas class="canvas" ref="canvas"></canvas>
  </div>
</template>

<style scoped>
.container {
  position: relative;
  width: 100vw;
  height: 100vh;
  overflow: hidden;
  animation: idle 2s ease infinite forwards alternate-reverse;
}
.canvas {
  overflow: hidden;
  transition: transform ease-out .6s;
}
@keyframes idle {
  from {
    transform: scale(1);
    opacity: .6;
  }
  to {
    transform: scale(1.002);
    opacity: 1;
  }
}
</style>
