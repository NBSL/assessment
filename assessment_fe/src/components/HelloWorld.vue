<template>
 <div class="container">
  <QuestionnaireComponent :question="questions[current].question" :grade="questions[current].grade" :next="questions[current].next" @nextQuestion="showNextQuestion" />
 </div>
</template>

<script lang="ts">
import { Component,Vue } from "vue-property-decorator";
import QuestionnaireComponent, {Grades} from "./Questionnaire.vue";

interface Questionnaire {
   question: string;
   grade: Grades;
   next: number | null;
}

@Component
export default class HelloWorld extends Vue {
  devices: any[] = [];
  questions: Questionnaire[] = [];
current = 0;
showNextQuestion(next: number|null) {
 if (next != null)
  this.current = next;
}

async setGrade(grade: Grades) {
let res = await fetch("http://localhost:3000/", {method: "POST", body: JSON.stringify({grade: grade})});
}

async getDevices() {
 let data  = await (await fetch("http://localhost:3000/devices")).json();
 this.devices = data;
}

async getQuestions() {
 let data  = await (await fetch("http://localhost:3000/questions")).json();
 this.questions = data;
 }

 mounted = () => {
   this.getQuestions();
  }
}

</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped></style>
