import { load_wasm, bind } from "./utils.ts"
import * as Canvas from "./canvas.ts"
import * as Sys from "./sys.ts"
import { EventType } from "./events.ts"

interface WASMExports {
  main(): void
  resize(width: number, height: number): void
  frame(): void
  pointer(event: EventType, x: number, y: number): void
  keyboard(event: EventType, keycode: number): void
}

export class WASM {

  ctx!: CanvasRenderingContext2D
  wasm!: WebAssembly.WebAssemblyInstantiatedSource

  get exports() {
    return this.wasm.instance.exports as unknown as WASMExports
  }

  get memory() {
    return this.wasm.instance.exports.memory as WebAssembly.Memory
  }

  gradients: (CanvasGradient|null)[] = []

  constructor() {
    this.onResize = this.onResize.bind(this)
    this.onFrame = this.onFrame.bind(this)
    this.onPointer = this.onPointer.bind(this)
    this.onKeyboard = this.onKeyboard.bind(this)
  }

  init() {
    // Init Canvas2D
    const $canvas = document.createElement('canvas')
    document.body.appendChild($canvas)
    $canvas.style.cssText = `position: fixed; top: 0; left: 0;`
    this.ctx = $canvas.getContext('2d') as CanvasRenderingContext2D
    $canvas.width = window.innerWidth
    $canvas.height = window.innerHeight
    $canvas.style.width = `${window.innerWidth}px`
    $canvas.style.height = `${window.innerHeight}px`

    // Initialize
    this.exports.main()

    // Listen resize event
    window.addEventListener("resize", this.onResize)
    window.addEventListener("pointerdown", this.onPointer)
    window.addEventListener("pointerup", this.onPointer)
    window.addEventListener("pointermove", this.onPointer)
    window.addEventListener("keyup", this.onKeyboard)
    window.addEventListener("keydown", this.onKeyboard)
    this.onResize()
    this.onFrame()
  }

  onResize() {
    this.ctx.canvas.width = window.innerWidth
    this.ctx.canvas.height = window.innerHeight
    this.ctx.canvas.style.width = `${window.innerWidth}px`
    this.ctx.canvas.style.height = `${window.innerHeight}px`
    this.exports.resize(Math.floor(this.ctx.canvas.width), Math.floor(this.ctx.canvas.height))
  }

  onFrame() {
    this.exports.frame()
    window.requestAnimationFrame(this.onFrame)
  }

  onPointer(e: PointerEvent) {
    switch (e.type) {
      case "pointerup":
        {
          this.exports.pointer(EventType.PointerUp, Math.floor(e.clientX), Math.floor(e.clientY))
          break
        }
      case "pointerdown":
        {
          this.exports.pointer(EventType.PointerDown, Math.floor(e.clientX), Math.floor(e.clientY))
          break
        }
      case "pointermove":
        {
          this.exports.pointer(EventType.PointerMove, Math.floor(e.clientX), Math.floor(e.clientY))
          break
        }
    }
  }

  onKeyboard(e: KeyboardEvent) {
    switch (e.type) {
      case "keyup":
        {
          this.exports.keyboard(EventType.KeyUp, e.keyCode)
          break
        }
      case "keydown":
        {
          if (!e.repeat) {
            this.exports.keyboard(EventType.KeyDown, e.keyCode)
          }
          this.exports.keyboard(EventType.KeyPressed, e.keyCode)
          break
        }
    }
  }

  static async load(path: string) {
    const i = new WASM()
    const env = bind({
      ...Sys,
      ...Canvas,
    }, i)
    const wasm = await load_wasm(path, { env })
    i.wasm = wasm
    i.init()
    return i
  }

}