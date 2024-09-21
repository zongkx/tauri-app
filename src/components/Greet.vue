<script setup>
import {ref} from "vue";
import {invoke} from "@tauri-apps/api/tauri";

const greetMsg = ref("");
const name = ref("");


async function greet() {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  greetMsg.value = await invoke("greet", {name: name.value});
}

</script>

<template>
  <div>
    <input v-model="endpoint" placeholder="Enter MinIO Endpoint"/>
    <input v-model="accessKey" placeholder="Enter Access Key"/>
    <input v-model="secretKey" placeholder="Enter Secret Key"/>
    <button @click="fetchBuckets">Fetch Buckets</button>
    <ul>
      <li v-for="bucket in buckets" :key="bucket.name">{{ bucket.name }}</li>
    </ul>
  </div>
</template>
<script>

export default {
  data() {
    return {
      endpoint: 'http://192.168.8.80:29000',
      accessKey: 'comen',
      secretKey: 'comen0755',
      buckets: []
    };
  },
  methods: {
    async fetchBuckets() {
      try {
        const config = {
          endpoint: this.endpoint,
          access_key: this.accessKey,
          secret_key: this.secretKey
        };
        const buckets = await invoke('list_buckets', {config});
        this.buckets = buckets;
      } catch (e) {
        console.error(e);
      }
    }
  }
}
</script>