<script lang="ts">
  import { invoke } from '@tauri-apps/api/tauri';
  import Screen from './lib/Screen.svelte'
  import Screenshot from './lib/Screenshot.svelte';
  import { TakeAreaScreenshot, TakeScreenshot } from './lib/TakeScreenshot';
  import { register, isRegistered } from '@tauri-apps/api/globalShortcut';

  import { open } from "@tauri-apps/api/dialog"
  import { convertFileSrc } from '@tauri-apps/api/tauri';



  let photoArray: [string];
  
  
  async function OnLoad() {
    if (!isRegistered('CommandOrControl+shift+c')){
      await register('CommandOrControl+shift+c', ()=> {
        invoke("open_overlay")
      })
    }
    photoArray = await invoke("get_photos")

  }

  OnLoad()

</script>

<main class="container flex flex-row max-w-full w-full text-text">
  <div class="sidebar h-full w-1/4 text-center bg-secondary">
    <!-- <img src="" alt=""> -->
    <h2 class="mt-40">Phoenix</h2>
    <button class="mt-20">button 1</button><br>
    <button class="mt-5">button 2</button>
  </div>
  <div class="imgshow p-4 bg-background w-full">

    {#if photoArray}
      {#each photoArray as PhotoPath}
        <Screenshot ImgPath={convertFileSrc(PhotoPath)} />
      {/each}
    {/if}
  </div>
  
  
  
  
</main>

<style lang="postcss">
.container {

  height: 100vh;
  /* width: 100vw; */
  
}

</style>