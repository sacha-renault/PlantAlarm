<template>
    <n-modal v-model:show="model" class="main-modal">
        <n-card>
            <n-flex verical align="center" justify="center">
                <h1> Add a new Plant !</h1>
                <n-divider />
                <n-form ref="formRef" :model="formValue" :rules="rules" class="wf">
                    <n-form-item class="form-item" label="Name" path="name">
                        <n-input v-model:value="formValue.name" placeholder="Input Name" />
                    </n-form-item>
                    <n-form-item class="form-item" label="Water once every" path="dayInterval">
                        <n-input-number v-model:value="formValue.dayInterval" clearable class="wf">
                            <template #suffix>
                                days
                            </template>
                        </n-input-number>
                    </n-form-item>
                    <n-form-item class="form-item" label="Water quantity" path="waterQty">
                        <n-input-number v-model:value="formValue.waterQty" clearable :step="50" class="wf">
                            <template #suffix>
                                mL
                            </template>
                        </n-input-number>
                    </n-form-item>
                    <n-form-item class="form-item" label="Last Watered" path="date">
                        <n-date-picker v-model:value="formValue.date" type="datetime" class="wf" />
                    </n-form-item>

                    <n-form-item class="form-item" label="Plant Image" path="image">
                        <n-flex vertical class="wf image-picker">
                            <input type="file" accept="image/*" capture="environment" @change="handleImageInput" />
                            <n-empty v-if="formValue.image === ''">
                                No file ...
                            </n-empty>
                            <n-flex v-else justify="center">
                                <n-avatar round :src="formValue.image" object-fit="cover" class="img-preview" />
                            </n-flex>
                        </n-flex>
                    </n-form-item>


                    <n-button-group class="wf" justify="space-around">
                        <n-button @click="handleValidateClick" primary type="success" style="width:50%">
                            Add plant
                        </n-button>
                        <n-button @click="resetForm" secondary type="error" style="width:50%">
                            Quit
                        </n-button>
                    </n-button-group>
                </n-form>
            </n-flex>
        </n-card>
    </n-modal>
</template>

<script setup lang="ts">
import type { FormInst } from 'naive-ui'
import { useMessage } from 'naive-ui';
import { ref } from 'vue';
import { api } from '../api';
import { PlantDto } from '../interfaces/dto';

const emits = defineEmits(['plantAdded']);
const message = useMessage();
const formRef = ref<FormInst | null>(null)
const model = defineModel({ default: false });
const formValue = ref({
    name: '',
    dayInterval: null,
    date: null,
    waterQty: null,
    image: ''
});
const rules = ref({
    name: {
        required: true,
        message: 'Please input your name',
        trigger: 'blur'
    },
    dayInterval: {
        required: true,
        type: 'number',
        message: 'Please input the day interval',
        trigger: ['input', 'blur']
    },
    date: {
        type: 'number',
        required: true,
        trigger: ['blur', 'change'],
        message: 'Please input last time the plant was watered'
    },
    waterQty: {
        required: true,
        type: 'number',
        message: 'Please input the water quantity (in mL)',
        trigger: ['input', 'blur']
    },
    image: {
        type: 'string'
    }
});

const resetForm = () => {
    formValue.value.date = null;
    formValue.value.dayInterval = null;
    formValue.value.waterQty = null;
    formValue.value.name = '';
    formValue.value.image = '';
    model.value = false;
}

const addPlant = (newPlant: PlantDto) => {
    api.addPlant(newPlant).then((plant) => {
        // plant created with success
        message.success('New plant created');
        emits('plantAdded', plant);

        if (formValue.value.date !== null) {
            console.log(formValue.value.date);
            api.addWatering(plant.id, formValue.value.date)
                .then(() => message.success('watering added'))
                .catch(err => message.error(err))
        }

        // Reset form should occure at the end ...
        resetForm();

    }).catch(err => {
        message.error('An error occured when creating new plant: ' + err);
    });
}

const handleValidateClick = (e: MouseEvent) => {
    e.preventDefault()
    formRef.value?.validate((errors) => {
        if (!errors && formValue.value.image !== '') {
            const newPlant: PlantDto = {
                name: formValue.value.name,
                dayInterval: formValue.value.dayInterval ?? 0, // Shouldn't be 0
                waterQuantity: formValue.value.waterQty ?? 0,  // Shouldn't be 0
                image: formValue.value.image
            };
            addPlant(newPlant);

        }
        else {
            message.error('Please fill all the inputs');
        }
    })
}

const handleImageInput = (e: Event) => {
    const target = e.target as HTMLInputElement;
    if (target.files && target.files.length > 0) {
        const file = target.files[0];
        const reader = new FileReader();
        reader.onload = (e) => {
            formValue.value.image = e.target?.result as string;
        };
        reader.readAsDataURL(file);
    }
}
</script>

<style scoped>
.form-item {
    margin: 0.25rem 1rem;
}


.img-preview {
    width: max(128px, 25%);
    height: auto;
    aspect-ratio: 1 / 1;
}
</style>