<script setup>
import { emit } from '@tauri-apps/api/event';
import { useManager } from '../stores/manager';

defineEmits(['add']);
const Manager = useManager();
const Open = async () => {
    Manager.OpenHosts();
}
const Save = async () => {
    Manager.SaveHosts();
}
const Fix = async () => {
    Manager.FixHosts();
}
</script>
<template>
    <div class="m-2 p-2 flex justify-content-between">
        <Button label="New Entry" @click="$emit('add')" />
        <div>
            <Button class="mx-1" icon="pi pi-wrench" rounded @click="Fix" />
            <Button class="mx-1" icon="pi pi-external-link" rounded @click="Open" />
            <Button class="mx-1" icon="pi pi-save" :class="{'bg-red-400 border-red-400': !Manager.ChangesSaved, 'bg-green-300 border-green-300': Manager.ChangesSaved}" rounded @click="Save" />
        </div>
    </div>
</template>