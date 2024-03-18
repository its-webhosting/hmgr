<script setup>
import { ref, onMounted } from 'vue';
import Manager from './components/Manager.vue';
import Navbar from './components/Navbar.vue';
import { useManager } from './stores/manager';
const FileManager = useManager();

const Entry = ref({
    ip: "",
    host: "",
    active: true,
    edit: false,
    index: 0
});

const AddEntry = () => {
    Entry.value.edit = false;
    Entry.value.ip = "";
    Entry.value.host = "";
    Entry.value.active = true;
    console.log(Entry);
    FileManager.ShowForm();
}

const EditEntry = (entry) => {
    console.log("Edit");
    Entry.value.edit = true;
    Entry.value.index = FileManager.Hosts.indexOf(entry);
    Entry.value.ip = entry.ip;
    Entry.value.host = entry.host;
    Entry.value.active = entry.active;
    
    FileManager.ShowForm();
}
</script>

<template>
  <Navbar @add="AddEntry"/>
  <Manager @edit="EditEntry" />

  <Dialog v-model:visible="FileManager.showDialog" modal :pt="{
        root: 'border-none',
        mask: {
            style: 'backdrop-filter: blur(2px)'
        }
    }">
        <template #container>
            <div class="flex flex-column px-5 py-5 gap-4"
                style="border-radius: 12px; background-image: radial-gradient(circle at left top, var(--primary-400), var(--primary-700))">
                <div class="inline-flex flex-column gap-2">
                    <label for="ip" class="text-primary-50 font-semibold">IP Address</label>
                    <InputText id="ip" v-model="Entry.ip" class="bg-white-alpha-20 border-none p-3 text-primary-50 w-20rem"></InputText>
                </div>
                <div class="inline-flex flex-column gap-2">
                    <label for="host" class="text-primary-50 font-semibold">Host</label>
                    <InputText id="host" v-model="Entry.host" class="bg-white-alpha-20 border-none p-3 text-primary-50 w-20rem"
                        type="text"></InputText>
                </div>
                <div class="flex align-items-center gap-3">
                    <Button label="Cancel" @click="FileManager.HideForm()" text
                        class="p-3 w-full text-primary-50 border-1 border-white-alpha-30 hover:bg-white-alpha-10"></Button>
                    <Button label="Finish" @click="FileManager.AddEntry(Entry)" text
                        class="p-3 w-full text-primary-50 border-1 border-white-alpha-30 hover:bg-white-alpha-10"></Button>
                </div>
            </div>
        </template>
    </Dialog>
</template>

<style scoped>
.logo.vite:hover {
  filter: drop-shadow(0 0 2em #747bff);
}

.logo.vue:hover {
  filter: drop-shadow(0 0 2em #249b73);
}
</style>
