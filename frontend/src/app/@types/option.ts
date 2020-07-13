export interface Option {
    data: string;
    type: 'Radio' | 'Checkbox';
    id: string;
    isCorrect: boolean;
}