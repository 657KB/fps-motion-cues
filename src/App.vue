<script setup lang="ts">
import { onMounted, onUnmounted, ref } from 'vue'
import { listen } from '@tauri-apps/api/event'
import { exit } from '@tauri-apps/api/process'
import { animate, arr, easeInOutSine, setLoop, stopLoop } from './utils'

type Key = { code: string }
type Mouse = { coords: [number, number] }
type Brightness = { brightness: [number, number, number] }

const canvas0 = ref<HTMLCanvasElement | null>(null)
const canvas1 = ref<HTMLCanvasElement | null>(null)
const canvas2 = ref<HTMLCanvasElement | null>(null)

const coef = {
  z: 400,
  pos: 1000,
  jumpTime: 400,
  jumpUp: 1.6,
  fallDown: -1,
}

const draw = {
  color: 'black',
}
const player = {
  position: { x: 0, y: 0, z: 0 },
  direction: { x: 0, y: 0, z: 0 },
  speed: 300,
}
const mouse = {
  coords: { x: 0, y: 0 },
  $position: { x: 0, y: 0 },
  position: { x: 0, y: 0 },
  direction: { x: 0, y: 0 },
  speed: 0,
  maxSpeed: 16,
  minSpeed: 10,
}

const coordsStatistics = new Map<string, number>()
const centerCoords = { x: 0, y: 0 }
let mostRecentlyCoordsCount = 0

const cancelAnimFn: Function[] = []

const exitKeyDown: string[] = []

const checkExit = () => {
  if (exitKeyDown.length === 3
    && exitKeyDown.findIndex(k => k === 'Q') !== -1
    && exitKeyDown.findIndex(k => k === 'LShift') !== -1
    && exitKeyDown.findIndex(k => k === 'LAlt') !== -1
  ) exit(1)
}

const keydown = listen<Key>('key_down', event => {
  const { code } = event.payload
  if (code === 'W' || code === 'Up') {
    player.direction.z += 1
  } else if (code === 'A' || code === 'Left') {
    player.direction.x += 1
  } else if (code === 'S' || code === 'Down') {
    player.direction.z -= 1
  } else if (code === 'D' || code === 'Right') {
    player.direction.x -= 1
  } else if (code === 'Space') {
    player.direction.y = coef.jumpUp
  } else if (code === 'F9') {
    mostRecentlyCoordsCount = 0
  } else if (code === 'Q') {
    exitKeyDown.push('Q')
    checkExit()
  } else if (code === 'LShift') {
    exitKeyDown.push('LShift')
    checkExit()
  } else if (code === 'LAlt') {
    exitKeyDown.push('LAlt')
    checkExit()
  }
})

const keyup = listen<Key>('key_up', event => {
  const { code } = event.payload
  if (code === 'W' || code === 'Up') {
    player.direction.z -= 1
  } else if (code === 'A' || code === 'Left') {
    player.direction.x -= 1
  } else if (code === 'S' || code === 'Down') {
    player.direction.z += 1
  } else if (code === 'D' || code === 'Right') {
    player.direction.x += 1
  } else if (code === 'Space') {
    setTimeout(() => {
      player.direction.y = coef.fallDown
      setTimeout(() => {
        player.direction.y = 0
      }, coef.jumpTime / 2)
    }, coef.jumpTime / 2)
  } else if (code === 'Q') {
    const i = exitKeyDown.findIndex(k => k === 'Q')
    exitKeyDown.splice(i, 1)
  } else if (code === 'LShift') {
    const i = exitKeyDown.findIndex(k => k === 'LShift')
    exitKeyDown.splice(i, 1)
  } else if (code === 'LAlt') {
    const i = exitKeyDown.findIndex(k => k === 'LAlt')
    exitKeyDown.splice(i, 1)
  }
})

const mousemove = listen<Mouse>('mouse_move', event => {
  const [x, y] = event.payload.coords
  if (mouse.coords.x === centerCoords.x && mouse.coords.y === centerCoords.y) {
    mouse.coords.x = x
    mouse.coords.y = y
    return
  }
  const diffX = x - mouse.coords.x
  const diffY = y - mouse.coords.y
  mouse.speed = Math.max(Math.min(Math.sqrt(Math.pow(diffX, 2) + Math.pow(diffY, 2)), mouse.maxSpeed), mouse.minSpeed)
  mouse.direction.x = diffX
  mouse.direction.y = diffY
  mouse.coords.x = x
  mouse.coords.y = y
})

const mouseidle = listen<Mouse>('mouse_idle', event => {
  const [x, y] = event.payload.coords
  const coordsKey = `${x}-${y}`
  const count = coordsStatistics.get(coordsKey) || 0
  coordsStatistics.set(coordsKey, count + 1)
  if (mostRecentlyCoordsCount < count + 1) {
    centerCoords.x = x
    centerCoords.y = y
    mostRecentlyCoordsCount = count + 1
  }
  if (coordsStatistics.size > 100) {
    for (const key in coordsStatistics.keys()) {
      if (key !== `${centerCoords.x}-${centerCoords.y}`) {
        coordsStatistics.delete(key)
      }
    }
  }
  requestAnimationFrame(() => {
    mouse.speed = 0
    mouse.direction.x = 0
    mouse.direction.y = 0
  })
})

const brightness = listen<Brightness>('screen_brightness', event => {
  const median = event.payload.brightness[1]
  if (median <= (255 / 2)) {
    draw.color = `white`
  } else if (median > 255 / 2) {
    draw.color = `black`
  }
})

const update = (delta: number) => {
  const canvasList = [canvas0.value, canvas1.value, canvas2.value]
  if (canvasList.some(c => c === null)) return

  if (mouse.speed !== 0) {
    cancelAnimFn.forEach(fn => fn())
    cancelAnimFn.splice(0, cancelAnimFn.length)
  }

  canvasList.forEach((canvas, canvasIndex) => {
    const ctx = canvas!.getContext('2d')
    const w = canvas!.width
    const h = canvas!.height

    if (ctx === null) return

    ctx.clearRect(0, 0, w, h)
    ctx.fillStyle = draw.color

    const pn = [player.direction.x, player.direction.y, player.direction.z].filter(d => d !== 0).length || 1
    player.position.x += (player.speed / pn / coef.pos) * delta * player.direction.x
    player.position.y += (player.speed / pn / coef.pos) * delta * player.direction.y
    player.position.z += (player.speed / pn / coef.pos) * delta * player.direction.z

    const mn = [mouse.direction.x, mouse.direction.y].filter(d => d !== 0).length || 1

    mouse.position.x = mouse.$position.x
    mouse.position.y = mouse.$position.y
    mouse.$position.x += (mouse.speed / mn / coef.pos) * delta * mouse.direction.x
    mouse.$position.y += (mouse.speed / mn / coef.pos) * delta * mouse.direction.y

    const cancel0 = animate({
      start: mouse.position.x,
      end: mouse.$position.x,
      duration: 1000,
      easeFn: easeInOutSine,
      update: v => {
        mouse.position.x = v
      },
    })
    const cancel1 = animate({
      start: mouse.position.y,
      end: mouse.$position.y,
      duration: 1000,
      easeFn: easeInOutSine,
      update: v => {
        mouse.position.y = v
      },
    })
    cancelAnimFn.push(cancel0)
    cancelAnimFn.push(cancel1)

    // z = 0 ~ 2
    const rawZ = player.position.z / coef.z + canvasIndex
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
          const posX = player.position.x + mouse.position.x
          const posY = player.position.y + mouse.position.y
          const offsetX = maxDistanceX * colIndex / 4
          const offsetY = h / (rowNum * 2) + (h / rowNum) * rowIndex
          const rawX = (posX + offsetX) % maxDistanceX + (side === 0 ? 0 : rightStartPosX)
          const rawY = (posY + offsetY) % maxDistanceY
          const x = posX < -offsetX ? maxDistanceX + rawX : rawX
          const y = posY < -offsetY ? maxDistanceY + rawY : rawY
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

onMounted(() => {
  resizeCanvas()
  window.addEventListener('resize', resizeCanvas)
  setLoop(update)
})

onUnmounted(() => {
  keydown.then(unlisten => unlisten())
  keyup.then(unlisten => unlisten())
  mousemove.then(unlisten => unlisten())
  mouseidle.then(unlisten => unlisten())
  brightness.then(unlisten => unlisten())
  window.removeEventListener('resize', resizeCanvas)
  stopLoop()
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
  mix-blend-mode: difference;
}
</style>
