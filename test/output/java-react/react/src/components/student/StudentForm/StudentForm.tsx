
    import React from 'react';
import { Box, TextField, Button, Grid, Typography } from '@mui/material';
import { useForm } from 'react-hook-form'; // 西方团队主流表单库
import { Student, StudentCreateRequest } from '../../../types/student.types';

interface StudentFormProps {
  initialData?: Partial<Student>; // 编辑时传初始值
  onSubmit: (student: StudentCreateRequest) => void;
  isSubmitting: boolean;
}

const StudentForm: React.FC<StudentFormProps> = ({ initialData, onSubmit, isSubmitting }) => {
  const { register, handleSubmit, formState: { errors } } = useForm<StudentCreateRequest>({
    defaultValues: initialData || {
      id:'',studentId:'',fullName:'',email:'',major:'',location:''
    },
  });

  return (
    <Box component="form" onSubmit={handleSubmit(onSubmit)} sx={{ mt: 2 }}>
      <Grid container spacing={2}>
      
<Grid item xs={12}>
<TextField
label="studentId"
{...register('studentId', {
required: 'Student ID is required (format: 2024XXX)',
maxLength: {value: 15, message: 'Student ID must be 8-15 characters'},
})}
error={!!errors.studentId}
/>
</Grid>
<Grid item xs={12}>
<TextField
label="fullName"
{...register('fullName', {
required: 'University email is required',
maxLength: {value: 60, message: 'Full name cannot exceed 60 characters'},
})}
error={!!errors.fullName}
/>
</Grid>
<Grid item xs={12}>
<TextField
label="email"
type="email"
{...register('email', {
required: 'University email is required',
})}
error={!!errors.email}
/>
</Grid>
<Grid item xs={12}>
<TextField
label="major"
{...register('major', {
maxLength: {value: 30, message: 'Major cannot exceed 30 characters'},
})}
/>
</Grid>
<Grid item xs={12}>
<TextField
label="location"
{...register('location', {
required: 'Family location is required',
})}
error={!!errors.location}
/>
</Grid>
        <Grid item xs={12}>
          <Button
            type="submit"
            variant="contained"
            fullWidth
            disabled={isSubmitting}
          >
            {isSubmitting ? 'Submitting...' : initialData ? 'Update Student' : 'Create Student'}
          </Button>
        </Grid>
      </Grid>
    </Box>
  );
};

export default StudentForm;
    