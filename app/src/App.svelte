<script>
  import Task from "./components/Task.svelte";
  import Wallet from "./components/Wallet.svelte";
  import {
    taskList,
    web3taskList,
    darkTheme,
    walletConnected,
    userAddress,
  } from "./stores/store.js";

  import toast, { Toaster } from "svelte-french-toast";

  const lightIcon = `<svg xmlns="http://www.w3.org/2000/svg" class="w-10 h-10 hover:fill-black dark:stroke-white dark:hover:fill-white" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor">
  <path stroke-linecap="round" stroke-linejoin="round" d="M12 3v2.25m6.364.386l-1.591 1.591M21 12h-2.25m-.386 6.364l-1.591-1.591M12 18.75V21m-4.773-4.227l-1.591 1.591M5.25 12H3m4.227-4.773L5.636 5.636M15.75 12a3.75 3.75 0 11-7.5 0 3.75 3.75 0 017.5 0z" /> </svg>`;
  const darkIcon = `<svg xmlns="http://www.w3.org/2000/svg" class="w-9 h-9 hover:fill-black dark:hover:fill-white" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor">
  <path stroke-linecap="round" stroke-linejoin="round" d="M21.752 15.002A9.718 9.718 0 0118 15.75c-5.385 0-9.75-4.365-9.75-9.75 0-1.33.266-2.597.748-3.752A9.753 9.753 0 003 11.25C3 16.635 7.365 21 12.75 21a9.753 9.753 0 009.002-5.998z" /> </svg>`;

  const addIcon = `<svg xmlns="http://www.w3.org/2000/svg" class="w-10 h-10 dark:fill-white" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor">
  <path stroke-linecap="round" stroke-linejoin="round" d="M12 9v6m3-3H9m12 0a9 9 0 11-18 0 9 9 0 0118 0z" /> </svg>`;

  const hamburgerIcon = `<svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-10 h-10 dark:stroke-white"> <path stroke-linecap="round" stroke-linejoin="round" d="M3.75 6.75h16.5M3.75 12h16.5m-16.5 5.25h16.5" /></svg>`;

  const closeIcon = `<svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-10 h-10 dark:stroke-white"> <path stroke-linecap="round" stroke-linejoin="round" d="M6 18L18 6M6 6l12 12" /> </svg>`;

  let input = "";
  let showHamburgerMenu = false;

  const addTask = () => {
    if (!$walletConnected) {
      $taskList = [
        ...$taskList,
        {
          taskID: $taskList.length,
          taskName: input,
          checked: false,
          deleted: false,
          disabled: true,
        },
      ];
      input = "";
    } else {
      // Transform into a promise for compatibility with Svelte French Toast
      const promise = new Promise(async (resolve, reject) => {
        const data = {
          address: $userAddress,
          taskID: $web3taskList.length,
          taskName: input,
          checked: false,
          deleted: false,
          disabled: true,
        };
        try {
          await fetch(
            import.meta.env.VITE_SERVER_URL + "/create",
            {
              method: "POST",

              headers: {
                "Content-Type": "application/json",
              },
              body: JSON.stringify(data),
            }
          );

          $web3taskList = [
            ...$web3taskList,
            {
              taskID: $web3taskList.length,
              taskName: input,
              checked: false,
              deleted: false,
              disabled: true,
            },
          ];
          input = "";
          resolve();
        } catch (error) {
          //error code for tx rejected by user - 4001
          reject();
        }
      });

      // svelte french toast promise notification
      toast.promise(
        promise,
        {
          loading: "Sending...",
          success: "Transaction was Succcessful",
          error: "Transaction Failed",
        },
        {
          position: "bottom-right",
        }
      );
    }
  };

  // unable to access body directly in svelte
  darkTheme.subscribe((value) => {
    if (value) {
      document.documentElement.classList.add("bg-[#181818]");
    } else {
      document.documentElement.classList.remove("bg-[#181818]");
    }
  });

  const switchTheme = () => {
    window.document.documentElement.classList.toggle("dark");
    $darkTheme = !$darkTheme;
  };
</script>

<nav
  class="w-screen text-center flex flex-row items-center justify-between p-2"
>
  <p class="font-medium font-serif text-4xl dark:text-white">to-do list</p>
  <div class="flex flex-row items-center gap-1">
    <Wallet CSSConfig="hidden md:block" />
    <button
      on:click={switchTheme}
      class="hidden md:block bg-transparent border-none m-5"
    >
      {#if $darkTheme}
        {@html lightIcon}
      {:else}
        {@html darkIcon}
      {/if}
    </button>
    <button
      on:click={() => {
        showHamburgerMenu = !showHamburgerMenu;
      }}
      class="md:hidden bg-transparent border-none"
    >
      {#if !showHamburgerMenu}
        {@html hamburgerIcon}
      {:else}
        {@html closeIcon}
      {/if}
    </button>
  </div>
</nav>

{#if showHamburgerMenu}
  <div
    class="w-full fixed bottom-0 left-0 mb-20 flex justify-between items-center md:hidden p-5"
  >
    <div class="">
      <Wallet CSSConfig="" />
    </div>
    <div class="flex justify-start mr-10">
      <button on:click={switchTheme} class="bg-transparent border-none">
        {#if $darkTheme}
          {@html lightIcon}
        {:else}
          {@html darkIcon}
        {/if}
      </button>
    </div>
  </div>
{:else}
  <div class="mx-auto p-5 flex justify-center gap-1">
    <div
      class="w-80 p-1 border-black border-2 dark:border-white dark:text-white shadow-lg font-medium placeholder:italic"
      bind:textContent={input}
      contenteditable="true"
      placeholder="Add task here"
    />
    <button on:click={addTask}>{@html addIcon}</button>
  </div>

  <div
    class="container mx-auto max-w-xl overflow-auto max-h-[66vh] scrollbar snap-y snap-mandatory [scroll-padding: 0px 0px calc(100% - 1.5rem) 0px;]"
  >
    {#if $walletConnected}
      {#each $web3taskList as task}
        {#if !task.checked && !task.deleted}
          <Task
            taskName={task.taskName}
            checked={task.checked}
            taskID={task.taskID}
            disabled={task.disabled}
          />
        {/if}
      {/each}
      {#each $web3taskList as task}
        {#if task.checked && !task.deleted}
          <p class="line-through dark:text-white">
            <Task
              taskName={task.taskName}
              checked={task.checked}
              taskID={task.taskID}
              disabled={task.disabled}
            />
          </p>
        {/if}
      {/each}
    {:else}
      {#each $taskList as task}
        {#if !task.checked && !task.deleted}
          <Task
            taskName={task.taskName}
            checked={task.checked}
            taskID={task.taskID}
            disabled={task.disabled}
          />
        {/if}
      {/each}
      {#each $taskList as task}
        {#if task.checked && !task.deleted}
          <p class="line-through dark:text-white">
            <Task
              taskName={task.taskName}
              checked={task.checked}
              taskID={task.taskID}
              disabled={task.disabled}
            />
          </p>
        {/if}
      {/each}
    {/if}
  </div>
{/if}
<Toaster />

