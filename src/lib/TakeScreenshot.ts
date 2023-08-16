import { invoke } from "@tauri-apps/api/tauri"

export async function TakeScreenshot() {
    await invoke("full_screenshot")
}

export async function TakeAreaScreenshot(xstart: number,ystart: number,width: number,height: number) {
    await invoke("area_screenshot", {xstart, ystart, width, height})
}