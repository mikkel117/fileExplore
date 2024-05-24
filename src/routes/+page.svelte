<script>
  import { invoke } from "@tauri-apps/api/tauri";
  import Greet from "../lib/Greet.svelte";
  import { onMount } from "svelte";

  onMount(async () => {
    invoke("get_standard_dirs");
  });
</script>

<Greet />
