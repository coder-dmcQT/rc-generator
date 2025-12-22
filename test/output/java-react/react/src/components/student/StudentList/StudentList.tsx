
import React from 'react';
import { Box, Table, TableBody, TableCell, TableHead, TableRow, Button, Typography } from '@mui/material';
// Material UI是西方团队主流UI库（替代AntD）
import { Student } from '../../../types/Student.types';
import { useStudentApi } from '../../../hooks/useStudentApi';

// 组件Props类型（西方团队强制定义）
interface StudentListProps {
  filterParams?: Record<string, string>;
  onEdit: (student: Student) => void;
  onView: (student: Student) => void;
}

const StudentList: React.FC<StudentListProps> = ({ filterParams, onEdit, onView }) => {
  const { useGetStudents, useDeleteStudent } = useStudentApi();
  const { data: students = [], isLoading, error } = useGetStudents(filterParams);
  const { mutate: deleteStudent, isPending: isDeleting } = useDeleteStudent();

  // 处理删除（西方团队强调确认提示）
  const handleDelete = (id: number) => {
    if (confirm('Are you sure you want to delete this Student?')) { // 西方风格确认语
      deleteStudent(id);
    }
  };

  if (isLoading) return <Typography>Loading students...</Typography>;
  if (error) return <Typography color="error">Error: {(error as Error).message}</Typography>;

  return (
    <Box sx={{ mt: 2 }}>
      <Table>
        <TableHead>
          <TableRow>
          <TableCell>id</TableCell>
<TableCell>studentId</TableCell>
<TableCell>fullName</TableCell>
<TableCell>email</TableCell>
<TableCell>major</TableCell>
<TableCell>location</TableCell>
            
            <TableCell>Actions</TableCell>
          </TableRow>
        </TableHead>
        <TableBody>
          {students.map((student) => (
            <TableRow key={student.id}>
              <TableCell>{student.id}</TableCell>
<TableCell>{student.studentId}</TableCell>
<TableCell>{student.fullName}</TableCell>
<TableCell>{student.email}</TableCell>
<TableCell>{student.major}</TableCell>
<TableCell>{student.location}</TableCell>
              <TableCell>
                <Button 
                  variant="outlined" 
                  size="small" 
                  onClick={() => onView(student)}
                  sx={{ mr: 1 }}
                >
                  View
                </Button>
                <Button 
                  variant="contained" 
                  size="small" 
                  onClick={() => onEdit(student)}
                  sx={{ mr: 1 }}
                >
                  Edit
                </Button>
                <Button 
                  variant="contained" 
                  color="error" 
                  size="small" 
                  onClick={() => handleDelete(student.id)}
                  disabled={isDeleting}
                >
                  Delete
                </Button>
              </TableCell>
            </TableRow>
          ))}
        </TableBody>
      </Table>
      {students.length === 0 && (
        <Typography sx={{ mt: 2 }}>No students found.</Typography>
      )}
    </Box>
  );
};

export default StudentList;
    