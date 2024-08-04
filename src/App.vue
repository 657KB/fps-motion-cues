<script setup lang="ts">
import { onMounted, onUnmounted, ref } from 'vue'
import { arr, setLoop } from './utils'
// import { event } from '@tauri-apps/api'
// import { listen } from '@tauri-apps/api/event'

// type Key = { code: string }
// type MouseMove = { coords: [number, number] }

const canvas0 = ref<HTMLCanvasElement | null>(null)
const canvas1 = ref<HTMLCanvasElement | null>(null)
const canvas2 = ref<HTMLCanvasElement | null>(null)

const position = { x: 0, y: 0, z: 0 }
const mouseCoords = { x: 0, y: 0 }

// The direction in which the player moves in the 3d coordinate axis
const moveDirection : { [k: string]: number } = { x: 0, y: 0, z: 0 }
// The direction of the mouse movement perspective
const viewDirection = { x: 0, y: 0 }

const moveSpeed = 2

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
  const canvasList = [canvas0.value, canvas1.value, canvas2.value]
  if (canvasList.some(c => c === null)) return

  canvasList.forEach((canvas, canvasIndex) => {
    const ctx = canvas!.getContext('2d')
    const w = canvas!.width
    const h = canvas!.height

    if (ctx === null) return

    ctx.clearRect(0, 0, w, h)
    ctx.fillStyle = '#000'

    const n = Object.keys(moveDirection).filter(k => moveDirection[k] !== 0).length || 1
    position.x += (moveSpeed / n / 10) * delta * moveDirection.x
    position.y += (moveSpeed / n / 10) * delta * moveDirection.y
    position.z += (moveSpeed / n / 10) * delta * moveDirection.z

    // z = 0 ~ 2
    const rawZ = position.z / 200 + canvasIndex
    const progressZ = rawZ < 0 ? (2 - Math.abs(rawZ % 2)) / 2 : (rawZ % 2) / 2
    const scale = 0.6 + progressZ
    const opacity = progressZ < 0.5 ? progressZ / 0.5 : (1 - progressZ) / 0.5

    canvas!.style.transform = `scale(${scale})`
    canvas!.style.opacity = `${opacity}`

    const maxDistanceX = w / 4
    const maxDistanceY = h
    const rightStartPosX = w - maxDistanceX
    const maxRadiusPosY = maxDistanceY / 2
    const maxLeftRadiusPosX = maxDistanceX / 2
    const maxRightRadiusPosX = rightStartPosX + maxDistanceX / 2
    const maxRadius = 12

    const drawSpot = (x: number, y: number, r: number) => {
      ctx.beginPath()
      ctx.arc(x, y, r, 0, Math.PI * 2, true)
      ctx.fill()
    }

    const leftProgress = (x: number, y: number) => {
      const progressY = y < maxRadiusPosY ? 1 - (maxRadiusPosY - y) / maxRadiusPosY : 1 - (y - maxRadiusPosY) / maxRadiusPosY
      const progressX = 1 - Math.abs(Math.abs(x / maxLeftRadiusPosX) - 1)
      return (1 - progressX) * progressX + progressX * progressY
    }

    const rightProgress = (x: number, y: number) => {
      const progressX = 1 - Math.abs((w - x) / (w - maxRightRadiusPosX) - 1)
      const progressY = y < maxRadiusPosY ? 1 - (maxRadiusPosY - y) / maxRadiusPosY : 1 - (y - maxRadiusPosY) / maxRadiusPosY
      return (1 - progressX) * progressX + progressX * progressY
    }

    // left, right
    arr(2).forEach((_, side) => {
      // column
      const colNum = 4
      arr(colNum).forEach((_, colIndex) => {
        // row
        const rowNum = colIndex % 2 === 0 ? 6 : 3
        arr(rowNum).forEach((_, rowIndex) => {
          const offsetX = maxDistanceX * colIndex / 4
          const offsetY = h / (rowNum * 2) + (h / rowNum) * rowIndex
          const rawX = (position.x + offsetX) % maxDistanceX + (side === 0 ? 0 : rightStartPosX)
          const rawY = (position.y + offsetY) % maxDistanceY
          const x = position.x < -offsetX ? maxDistanceX + rawX : rawX
          const y = position.y < -offsetY ? maxDistanceY + rawY : rawY
          drawSpot(x, y, side === 0 ? maxRadius * leftProgress(x, y) : maxRadius * rightProgress(x, y))
        })
      })
    })
  })
}

const resizeCanvas = () => {
  if (canvas0.value !== null) {
    canvas0.value.width = document.body.clientWidth
    canvas0.value.height = document.body.clientHeight
  }
  if (canvas1.value !== null) {
    canvas1.value.width = document.body.clientWidth
    canvas1.value.height = document.body.clientHeight
  }
  if (canvas2.value !== null) {
    canvas2.value.width = document.body.clientWidth
    canvas2.value.height = document.body.clientHeight
  }
}

const handleKeyDown = (event: KeyboardEvent) => {
  const { code } = event
  if (code === 'KeyW') {
    moveDirection.z += 1
  } else if (code === 'KeyA') {
    moveDirection.x += 1
  } else if (code === 'KeyS') {
    moveDirection.z -= 1
  } else if (code === 'KeyD') {
    moveDirection.x -= 1
  }
  if (moveDirection.x > 1) {
    moveDirection.x = 1
  } else if (moveDirection.x < -1) {
    moveDirection.x = -1
  }
  if (moveDirection.z > 1) {
    moveDirection.z = 1
  } else if (moveDirection.z < -1) {
    moveDirection.z = -1
  }
}
const handleKeyUp = (event: KeyboardEvent) => {
  const { code } = event
  if (code === 'KeyW') {
    moveDirection.z -= 1
  } else if (code === 'KeyA') {
    moveDirection.x -= 1
  } else if (code === 'KeyS') {
    moveDirection.z += 1
  } else if (code === 'KeyD') {
    moveDirection.x += 1
  }
  if (moveDirection.x > 1) {
    moveDirection.x = 1
  } else if (moveDirection.x < -1) {
    moveDirection.x = -1
  }
  if (moveDirection.z > 1) {
    moveDirection.z = 1
  } else if (moveDirection.z < -1) {
    moveDirection.z = -1
  }
}
const handlePointerMove = (event: PointerEvent) => {
  const { screenX, screenY } = event
  mouseCoords.x = screenX
  mouseCoords.y = screenY
}

onMounted(() => {
  resizeCanvas()
  window.addEventListener('keydown', handleKeyDown)
  window.addEventListener('keyup', handleKeyUp)
  window.addEventListener('pointermove', handlePointerMove)
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
  window.removeEventListener('pointermove', handlePointerMove)
  window.removeEventListener('resize', resizeCanvas)
})
</script>

<template>
  <div class="container">
    <canvas class="canvas" ref="canvas0"></canvas>
    <canvas class="canvas" ref="canvas1"></canvas>
    <canvas class="canvas" ref="canvas2"></canvas>
  </div>
</template>

<style scoped>
.container {
  position: relative;
  width: 100vw;
  height: 100vh;
}
.canvas {
  position: absolute;
  top: 0;
  left: 0;
}
</style>
