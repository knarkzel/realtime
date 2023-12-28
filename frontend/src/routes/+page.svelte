<script lang="ts">
  import { io } from "$lib/socket";
  import { onMount } from "svelte";

  // State
  let messages = [];
  let textfield = "";

  // Helper
  function sendMessage() {
    io.emit("message", textfield.trim());
    textfield = "";
  }

  onMount(() => {
    io.emit("join", "default");
    
    io.on("response", (message) => {
      messages = [...messages, message];
    });
  });
</script>

<div class="container mx-auto h-screen flex justify-between flex-col">
  <div>
    {#each messages as message}
      <p>{message}</p>
    {/each}
  </div>

  <form class="flex" on:submit|preventDefault={sendMessage}>
    <input class="input" type="text" bind:value={textfield} autofocus />
    <button type="submit" class="btn variant-filled-secondary">Send</button>
  </form>
</div>
