<script>
  import {
    web3taskList,
    taskList,
    walletConnected,
    userAddress,
  } from "../stores/store";
  import toast, { Toaster } from "svelte-french-toast";

  // Task Attributes
  export let taskID;
  export let taskName;
  export let checked;
  export let disabled; // Used to set caret-color as transparent or auto (disabled can't be used for div)
  let temp = undefined;

  const deleteIcon = `<svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6 hover:fill-red-500/60 dark:stroke-red-500" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
  <path stroke-linecap="round" stroke-linejoin="round" d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16" />
</svg>`;

  const editIcon = `<svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6 hover:fill-yellow-500/60 dark:stroke-yellow-500" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
  <path stroke-linecap="round" stroke-linejoin="round" d="M15.232 5.232l3.536 3.536m-2.036-5.036a2.5 2.5 0 113.536 3.536L6.5 21.036H3v-3.572L16.732 3.732z" />
</svg>`;

  const checkIcon = `<svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6 hover:fill-green-500/60 dark:stroke-green-500" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
  <path stroke-linecap="round" stroke-linejoin="round" d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z" />
</svg>`;

  const updateTask = () => {
    if (!$walletConnected) {
      $taskList[taskID].checked = !$taskList[taskID].checked;
    } else {
      // web3 update task
      const data = {
        address: $userAddress,
        taskID: taskID,
        checked: !$web3taskList[taskID].checked,
      };
      const promise = new Promise(async (resolve, reject) => {
        try {
          await fetch(import.meta.env.VITE_SERVER_URL + "/update", {
            method: "POST",

            headers: {
              "Content-Type": "application/json",
            },
            body: JSON.stringify(data),
          });

          $web3taskList[taskID].checked = !$web3taskList[taskID].checked;
          resolve();
        } catch (error) {
          //error code for tx rejected by user - 4001
          checked = !checked;
          reject();
        }
      });

      // svelte french toast notification
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

  const deleteTask = () => {
    if (!$walletConnected) {
      $taskList[taskID].deleted = true;
    } else {
      // web3 delete task
      const data = {
        address: $userAddress,
        taskID: taskID,
        deleted: true,
      };
      const promise = new Promise(async (resolve, reject) => {
        try {
          await fetch(import.meta.env.VITE_SERVER_URL + "/update", {
            method: "POST",
            headers: {
              "Content-Type": "application/json",
            },
            body: JSON.stringify(data),
          });
          $web3taskList[taskID].deleted = true;
          resolve();
        } catch (error) {
          //error code for tx rejected by user - 4001
          reject();
        }
      });

      // svelte french toast notification
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

  const editTask = () => {
    if (!$walletConnected) {
      disabled = true;
      $taskList[taskID].taskName = taskName;
    } else {
      // web3 edit task
      const data = {
        address: $userAddress,
        taskID: taskID,
        taskName: taskName,
      };
      const promise = new Promise(async (resolve, reject) => {
        try {
          await fetch(import.meta.env.VITE_SERVER_URL + "/update", {
            method: "POST",

            headers: {
              "Content-Type": "application/json",
            },
            body: JSON.stringify(data),
          });
          resolve();
        } catch (error) {
          //error code for tx rejected by user - 4001
          reject();
        }
      });

      promise
        .then(() => {
          disabled = true;
          $web3taskList[taskID].taskName = taskName;
        })
        .catch(() => {
          disabled = true;
          taskName = temp;
          $web3taskList[taskID].taskName = taskName;
        });

      // svelte french toast notification
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
</script>

<div
  class="mx-auto my-4 md:max-w-sm max-w-xs flex items-center justify-between rounded-lg p-5 text-left shadow-lg ring-1 ring-black/5 dark:bg-[#404040] border-2 border-black dark:border-white snap-end snap-always last:[margin-bottom: calc(100% - 100vh + (100vh - 100% + var(--scrollbar-width, 16px)));] mb-5"
>
  <div class="flex items-center gap-2">
    <input
      class="block rounded-full w-5 h-5 p-1.25 accent-black font-medium"
      type="checkbox"
      bind:checked
      on:change={updateTask}
    />

    <div
      class="w-[20ch] md:w-[26ch] outline-none {disabled
        ? ''
        : 'border-2 border-green-400'} text-slate-900 dark:text-white font-medium disabled:bg-white dark:bg-[#404040]
       [caret-color:{disabled ? 'transparent' : 'auto'}]"
      bind:textContent={taskName}
      contenteditable="true"
    />
  </div>

  <div class="flex gap-2">
    {#if disabled}
      <button
        on:click={() => {
          disabled = false;
          temp = taskName;
        }}
        class="p-0 bg-transparent border-none">{@html editIcon}</button
      >
    {:else}
      <button on:click={editTask} class=" p-0 bg-transparent border-none"
        >{@html checkIcon}</button
      >
    {/if}
    <button on:click={deleteTask} class=" p-0 bg-transparent border-none">
      {@html deleteIcon}</button
    >
  </div>
</div>
<Toaster />
