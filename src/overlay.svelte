<script lang="ts">
  import { convertFileSrc, invoke } from "@tauri-apps/api/tauri";
  import { appWindow } from '@tauri-apps/api/window';

  let BoxStart = {x:0, y: 0}
  let BoxEnd = {x:10, y: 10}
  let CurrentMouse = {x: 0, y: 0}

  let ShowBox = false
  let UpdateEnd = true



  function HandleMouse(event){
    CurrentMouse.x = event.clientX;
		CurrentMouse.y = event.clientY;
    if (UpdateEnd) {
      BoxEnd = CurrentMouse
    }
  }

  function MouseDown() {
    BoxStart.x = CurrentMouse.x
    BoxStart.y = CurrentMouse.y
    console.log("Click")
    ShowBox = true
  }

  async function MouseUp() {
    UpdateEnd = false
    await invoke("area_screenshot", {xstart: BoxStart.x, ystart: BoxStart.y, width: BoxEnd.x-BoxStart.x, height: BoxEnd.y-BoxStart.y})
    appWindow.close()
  }

</script>


<main class="container"  on:mousemove={HandleMouse} style="background-image: url({convertFileSrc("/home/funky/Pictures/Phoenix/temp.png")});">
  <!-- svelte-ignore a11y-no-static-element-interactions -->
    <div class="grey" on:mousedown={MouseDown} on:mouseup={MouseUp} >
    {#if ShowBox}
      <div class="box" style="top: {BoxStart.y}px; left: {BoxStart.x}px; height: {BoxEnd.y-BoxStart.y}px; width: {BoxEnd.x-BoxStart.x}px; 
      background-image: url({convertFileSrc("/home/funky/Pictures/Phoenix/temp.png")});background-position: -{BoxStart.x}px -{BoxStart.y}px;"> 
      </div>
    {/if}
  </div>
</main>

<style>
:root {
  font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
  font-size: 16px;
  line-height: 24px;
  font-weight: 400;
  margin: 0;


  color: #0f0f0f;

  font-synthesis: none;
  text-rendering: optimizeLegibility;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  -webkit-text-size-adjust: 100%;
}
.container {
  margin: 0;
  padding: 0;
  display: flex;
  flex-direction: column;
  justify-content: center;
  text-align: center;
  height: 100vh;
  color: black;

}
.grey {
  margin: 0;
  padding: 0;
  background-color: rgba(250, 250, 250, 0.8);
  width: 100%;
  height: 100%;
}
.box {
  position: absolute;
  min-width: 10px;
  min-height: 10px;
}
</style>