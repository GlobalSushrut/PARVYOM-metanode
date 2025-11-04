/**
 * Test page to preview the Email Verification / OTP page
 * Access at: http://localhost:5173/test-otp
 */

import React from 'react';
import EmailVerification from '../components/Auth/EmailVerification';

const TestOTP: React.FC = () => {
  return (
    <EmailVerification
      email="test@example.com"
      onVerificationSuccess={() => {
        console.log('Verification successful!');
        alert('Verification successful! (This is a test)');
      }}
      onBackToSignup={() => {
        console.log('Back to signup clicked');
        alert('Back to signup (This is a test)');
      }}
    />
  );
};

export default TestOTP;
