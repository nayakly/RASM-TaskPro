<script>
  import {
    walletConnected,
    web3taskList,
    userAddress,
  } from "../stores/store.js";

  import { Web3Provider } from "@ethersproject/providers";
  import toast, { Toaster } from "svelte-french-toast";

  export let CSSConfig = "";
  let walletMessage = "Switch to Web3"; // Switch to Web3 | Go Back to Web2

  const connectWallet = async () => {
    if (!$walletConnected) {
      if (window.ethereum === undefined) {
        //No Metamask wallet detected notification
        toast.error("No Ethereum Wallet detected. Please install Metamask", {
          position: "bottom-right",
        });
      } else {
        let chainId = parseInt(
          await window.ethereum.request({ method: "eth_chainId" }),
          16
        );

        if (chainId !== 5) {
          //Network Error - Switch user to the right network
          toast.error("Wrong Network! Please switch Metamask to Goerli", {
            position: "bottom-right",
          });
          try {
            // Switch user to the right network
            await window.ethereum.request({
              method: "wallet_switchEthereumChain",
              params: [{ chainId: "0x5" }],
            });
          } catch (error) {
            toast.error("Switch Rejected by User", {
              position: "bottom-right",
            });
          }
        } else {
          // Connect user's wallet with the contract. Save user address and balances
          const provider = new Web3Provider(window.ethereum);
          await provider.send("eth_requestAccounts", []);
          const signer = provider.getSigner(0);
          $userAddress = await signer.getAddress();

          // Retrieve Tasks From MongoDB
          try {
            const data = { address: $userAddress };

            const response = await fetch(
              import.meta.env.VITE_SERVER_URL + "/read",
              {
                method: "POST",
                headers: {
                  "Content-Type": "application/json",
                },
                body: JSON.stringify(data),
              }
            );
            const json = await response.json();
            const record = JSON.parse(json);
            $web3taskList = record["data"];
            toast.success("Wallet Connected", {
              position: "bottom-right",
            });
          } catch (error) {
            toast.error("Server Error", {
              position: "bottom-right",
            });
            throw error;
          }
          walletMessage = "Go back to Web2";
          $walletConnected = true;
        }
      }
    } else {
      $web3taskList = [];
      $userAddress = undefined;
      walletMessage = "Switch to Web3";
      $walletConnected = false;

      toast.success("Switched to Web2", {
        position: "bottom-right",
      });
    }
  };
</script>

<button class="relative text-lg group {CSSConfig}" on:click={connectWallet}>
  <span
    class="relative z-10 block px-5 py-3 overflow-hidden font-medium leading-tight text-gray-800 transition-colors duration-300 ease-out border-2 border-gray-900 rounded-lg group-hover:text-white"
  >
    <span
      class="absolute inset-0 w-full h-full px-5 py-3 rounded-lg bg-white"
    />
    <span
      class="absolute left-0 w-48 h-48 -ml-2 transition-all duration-300 origin-top-right -rotate-90 -translate-x-full translate-y-12 bg-gray-900 group-hover:-rotate-180 ease"
    />
    <span class="relative">{walletMessage}</span>
  </span>
  <span
    class="absolute bottom-0 right-0 w-full h-12 -mb-1 -mr-1 transition-all duration-200 ease-linear bg-gray-900 dark:bg-white rounded-lg group-hover:mb-0 group-hover:mr-0"
    data-rounded="rounded-lg"
  />
</button>

<Toaster />
