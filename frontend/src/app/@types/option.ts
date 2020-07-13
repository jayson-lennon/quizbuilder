export interface Option {
    data: string;
    type: 'SingleChoice' | 'MultiChoice';
    id: string;
    isCorrect: boolean;
}