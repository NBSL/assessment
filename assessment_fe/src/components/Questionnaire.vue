<template>
 <div>
  <p>
   {{ question }}
  </p>
  <button @click="$emit('setGrade', grade)">Yes</button>
  <button @click="$emit('nextQuestion', next)">No</button>
 </div>
</template>

<script lang="ts">
import {Component, Prop} from 'vue-property-decorator';
import Vue from 'vue/types/umd';

export type Grades = "A" | "A+" | "B" | "B+" | "C" | "C+" | "D" ;

@Component
export default class QuestionnaireComponent extends Vue {
@Prop() question!: String;
@Prop() grade!: Grades;
@Prop() next!: number | null;
}


</script>