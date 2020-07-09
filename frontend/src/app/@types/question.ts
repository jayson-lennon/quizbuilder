import { Option } from './option';

export interface Question {
    data: string;
    options: Option[];
    id: string;
}