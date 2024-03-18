<script setup>
// On Mounted, we get all the hosts files and set the reference
import { ref, onMounted } from "vue";
import { readTextFile } from "@tauri-apps/api/fs";
import { useManager } from "../stores/manager";
import Skeleton from "primevue/skeleton";

const Manager = useManager();
const selectedEntry = ref({
  ip: "",
  host: "",
  active: false,
});

const emit = defineEmits(["edit"]);

// Load Functions
onMounted(async () => {
  LoadFile();
});

// Load File
const LoadFile = async () => {
  if(Manager.HFL === "") {
    setTimeout(LoadFile, 100)
    return;
  }
  const hosts = await readTextFile(Manager.HFL);
  Manager.SetHostsFromString(hosts);
};

// Context Menu
const menu = ref();
const menuItems = ref([
  {
    label: "Edit",
    icon: "pi pi-pencil",
    command: () => {
      emit("edit", selectedEntry.value);
    },
  },
  {
    label: "Delete",
    icon: "pi pi-trash",
    command: () => {
      Manager.DeleteEntry(selectedEntry.value);
    },
  },
]);

const managementMenu = (event, entry) => {
  selectedEntry.value = entry;
  console.log(selectedEntry.value);
  menu.value.show(event);
};

// CRUD Functions
const SaveChanges = () => {
  // Add or Update
  console.log(editEntry.value);
  console.log(selectedEntry.value);

  editEntry.value = true;
};

const UndoChanges = () => {
  selectedEntry.value = {
    ip: "",
    host: "",
    active: false,
  };
  Manager.HideForm();
};
</script>

<template>
  <!-- Hosts File Entries -->
  <div class="surface-card m-2 p-2 shadow-2 border-round">
    <ul class="list-none p-0 m-0">
      <li
        class="surface-border"
        v-for="entry in Manager.Hosts"
        v-if="Manager.Hosts.length > 0"
        @contextmenu="managementMenu($event, entry)"
      >
        <div
          class="border-round surface-0 hover:surface-100 p-1 my-1 mx-1 grid"
          :class="{ 'surface-200': entry === selectedEntry }"
        >
          <div class="col-4 flex align-items-center">
            {{ entry.ip }}
          </div>
          <div class="col-6 flex align-items-center">
            {{ entry.host }}
          </div>
          <div class="col-2 flex align-items-center">
            <InputSwitch v-model="entry.active" />
          </div>
        </div>
      </li>
      <li v-else>
        <Skeleton height="4rem" />
      </li>
    </ul>
  </div>
  <!-- Context Menu -->
  <ContextMenu ref="menu" :model="menuItems" @hide="selectedEntry = null">
    <template #item="{ item, props }">
      <a v-ripple class="flex align-items-center" v-bind="props.action">
        <span :class="item.icon" />
        <span class="ml-2">{{ item.label }}</span>
        <Badge v-if="item.badge" class="ml-auto" :value="item.badge" />
        <span
          v-if="item.shortcut"
          class="ml-auto border-1 surface-border border-round surface-100 text-xs p-1"
        >
          {{ item.shortcut }}
        </span>
        <i v-if="item.items" class="pi pi-angle-right ml-auto"></i>
      </a>
    </template>
  </ContextMenu>
</template>
