<script lang="ts">
  import { invoke } from '@tauri-apps/api/tauri';
  import Screen from './lib/Screen.svelte'
  import { TakeAreaScreenshot, TakeScreenshot } from './lib/TakeScreenshot';
  import { register } from '@tauri-apps/api/globalShortcut';

  import { open } from "@tauri-apps/api/dialog"
  import { convertFileSrc } from '@tauri-apps/api/tauri';



  let photoArray: [string];
  
  
  async function OnLoad() {
    const isRegistered = await register('CommandOrControl+shift+c', ()=> {
      invoke("open_overlay")
    });
    photoArray = await invoke("get_photos")

  }

</script>

<main class="container">
  <h1>Phoenix screenshots</h1>


  <button on:click={()=> { invoke("open_overlay")}}>Take area screenshot</button>
  

 

  
  {#if photoArray}
    {#each photoArray as PhotoPath}
      <img src="{convertFileSrc(PhotoPath)}" alt="">
    {/each}
  {/if}
  
  

</main>

<style>
</style>